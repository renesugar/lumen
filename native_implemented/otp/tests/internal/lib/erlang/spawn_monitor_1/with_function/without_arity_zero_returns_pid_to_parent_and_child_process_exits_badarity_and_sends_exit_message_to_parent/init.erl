-module(init).
-export([start/0]).
-import(erlang, [display/1]).
-import(lumen, [log_exit/1]).

start() ->
  log_exit(false),
  {_ChildPid, ChildMonitorReference} = spawn_monitor(fun (A) ->
    display(A)
  end),
  receive
    {'DOWN', ChildMonitorReference, process, _, {Reason, Details}} ->
      display({child, exited, Reason})
    after 10 ->
      display(timeout)
  end.
