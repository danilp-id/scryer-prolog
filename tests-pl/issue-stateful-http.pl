% Server
:- use_module(library(process)).
:- use_module(library(iso_ext)).
:- use_module(library(os)).

prolog_path(Prolog) :-
    read(Body),
    term_variables(Body, [Prolog]),
    Body.

server_start([Process,Out]) :-
    prolog_path(Prolog),
    process_create(Prolog,
        ["tests-pl/issue-stateful-http_server", "-t", "run"],
        [process(Process), stdout(pipe(Out))]).

server_wait_start([_Process, Out]) :-
    get_char(Out, _C).

server_stop([Process,_Out]) :-
    process_kill(Process).

% Client 
:- use_module(library(charsio)).
:- use_module(library(http/http_open)).

send_request(Path) :-
    Options = [
        method('get'),
        status_code(StatusCode),
        request_headers([]),
        headers(_)
    ],
    append("http://localhost:8472/", Path, URL),
    http_open(URL, Stream, Options),
    get_line_to_chars(Stream,Line,[]),
    write_term(Line, []),nl.

main :-
    setup_call_cleanup(
        server_start(Server),
        (
            server_wait_start(Server),
            send_request,
            send_request,
            send_request,
            send_request,
            send_request
        ),
        server_stop(Server)
    ).

:- initialization(main).