-module(erlang_server).
-export([main/1]).

-define(ADDRESS, "ipc:///tmp/a.ipc").


main([]) ->
    io:format("OS pid: ~s~n", [os:getpid()]),
    enm:start_link(),

    io:format("Connecting to address ~s~n", [?ADDRESS]),
    {ok, Socket} = enm:req([{connect, ?ADDRESS}]),

    send_expect(<<"PING">>, <<"PONG">>, Socket),
    send_expect(<<"STOP">>, <<"OK">>, Socket),
    ok.


send_expect(Send, Expect, Socket) ->
    io:format("Sending '~s'~n", [Send]),
    ok = enm:send(Socket, Send),

    receive
        {nnreq, Socket, Message} -> 
            Unexpected = if Message =:= Expect -> "";
                            true -> " UNEXPECTED"
                         end,
            io:format("Received~s '~s'~n", [Unexpected, Expect])
    end.

