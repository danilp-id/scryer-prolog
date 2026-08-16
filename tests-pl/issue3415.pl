:- use_module(library(http/http_server)).

main :-
    Addr = "0.0.0.0:8475",
    '$http_listen'(Addr, Listener, "", "", 32768),
    (   '$http_listen'(Addr, _, "", "", 32768) ->
        write(listened_twice)
    ;   write(second_listen_failed)
    ),
    nl,
    '$http_listen_stop'(Listener).

:- initialization(main).
