# Labels 3ds CTR lib handles
# @category 3ds
#
# Patched for headless + XY: upstream raises when ctr::srv::GetServiceHandleDirect
# is not labeled. XY (2013, C-API-era SDK) has no such symbol, so the script
# now prints a skip notice instead of raising, keeping the headless postScript
# chain alive for the scripts that follow.

from ghidra.program.model.symbol import SourceType
from ghidra_utils import getOrCreateNamespace, getCallArgsFromRef
from ctr_services import getServiceHandleName, getServiceCallerRefs

memory = currentProgram.getMemory()

# Find service refs
service_caller_refs = getServiceCallerRefs()

# Find service handles
ctr_namespace = getOrCreateNamespace('ctr')
srv_namespace = getOrCreateNamespace('srv', ctr_namespace)
possible_get_service_handle_direct_symbols = getSymbols('GetServiceHandleDirect', srv_namespace)

if len(possible_get_service_handle_direct_symbols) == 0:
  print('SKIPPED Label3dsHandles: ctr::srv::GetServiceHandleDirect is not labeled in this '
        + 'program. XY uses the C-era srvGetServiceHandle API, so this C++-API script has '
        + 'nothing to anchor on. Label it manually to enable IPC handle attribution.')
else:
  if len(possible_get_service_handle_direct_symbols) > 1:
    print('SKIPPED Label3dsHandles: more than one ctr::srv::GetServiceHandleDirect symbol.')

  get_service_handle_direct = possible_get_service_handle_direct_symbols[0]
  get_handle_refs = getReferencesTo(get_service_handle_direct.getAddress())

  service_handles = {}

  for get_handle_ref in get_handle_refs:
    addr = get_handle_ref.getFromAddress()
    args = getCallArgsFromRef(get_handle_ref)
    if len(args) < 2:
      print('Bad number of args at {}'.format(addr))
      continue
    handle_ref = args[0]
    service = args[1]
    if handle_ref.isAddress():
      handle_addr = memory.getInt(handle_ref.getAddress())
      handle = toAddr(handle_addr)
      caller_func = getFunctionContaining(addr)
      if caller_func is not None:
        caller_addr = caller_func.getEntryPoint()
        service = service_caller_refs[caller_addr]
        service_handles[service] = handle
        label = getServiceHandleName(service)
        createLabel(handle, label, ctr_namespace, True, SourceType.USER_DEFINED)
        print('Created handle {} at {}'.format(label, handle))
    else:
      # TODO: This might be improved with pcode
      print('Unable to detect handle name at {}'.format(addr))
