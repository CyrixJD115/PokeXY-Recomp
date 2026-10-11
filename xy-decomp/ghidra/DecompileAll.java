import ghidra.app.script.GhidraScript;
import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileOptions;
import ghidra.app.decompiler.DecompileResults;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSet;
import ghidra.program.model.listing.Function;

import java.io.BufferedWriter;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

/**
 * Decompile every function in the program to one .c file each, plus a
 * functions.tsv manifest (entry, name, body size).
 *
 * Usage (postScript): DecompileAll.java <output-dir>
 */
public class DecompileAll extends GhidraScript {

    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length < 1) {
            throw new IllegalArgumentException("usage: DecompileAll.java <output-dir>");
        }
        Path outDir = Paths.get(args[0]);
        Files.createDirectories(outDir);

        DecompInterface ifc = new DecompInterface();
        ifc.setOptions(new DecompileOptions());
        ifc.openProgram(currentProgram);

        // Tiny stub modules (172–220-byte debug/test CROs) have no entry
        // points, so auto-analysis creates zero functions and the loop below
        // would emit an empty manifest. Force one function spanning the whole
        // image — these modules are single template handlers anyway.
        if (!currentProgram.getFunctionManager().getFunctions(true).hasNext()) {
            Address base = currentProgram.getMinAddress();
            disassemble(base);
            Function forced = createFunction(base, "module_entry");
            if (forced != null) {
                forced.setBody(new AddressSet(
                        currentProgram.getMinAddress(), currentProgram.getMaxAddress()));
                println("DecompileAll: zero functions; forced module_entry @ " + base);
            }
        }

        Path manifest = outDir.resolve("functions.tsv");
        long ok = 0, failed = 0;
        try (BufferedWriter m = Files.newBufferedWriter(manifest)) {
            m.write("entry\tname\tbody_size\tdecompiled\n");
            for (Function f : currentProgram.getFunctionManager().getFunctions(true)) {
                if (monitor.isCancelled()) {
                    break;
                }
                String entry = f.getEntryPoint().toString();
                String name = sanitize(f.getName());
                String c = null;
                try {
                    DecompileResults res = ifc.decompileFunction(f, 90, monitor);
                    if (res != null && res.getDecompiledFunction() != null) {
                        c = res.getDecompiledFunction().getC();
                    }
                } catch (Exception e) {
                    // leave c == null; recorded as failure below
                }
                boolean good = c != null;
                if (good) {
                    ok++;
                } else {
                    failed++;
                    c = "// decompilation failed\n";
                }
                Path file = outDir.resolve(entry + "_" + name + ".c");
                Files.writeString(file, "// " + name + " @ " + entry + "\n" + c);
                m.write(entry + "\t" + name + "\t"
                        + f.getBody().getNumAddresses() + "\t" + good + "\n");
            }
        }
        ifc.dispose();
        println("DecompileAll: " + ok + " ok, " + failed + " failed -> " + outDir);
    }

    private static String sanitize(String s) {
        return s.replaceAll("[^A-Za-z0-9_]", "_");
    }
}
