% Server
:- use_module(library(process)).
:- use_module(library(iso_ext)).
:- use_module(library(os)).
:- use_module(library(lists)).

prolog_path(Prolog) :-
    read(Body),
    term_variables(Body, [Prolog]),
    Body.

server_start([Process,Out]) :-
    prolog_path(Prolog),
    process_create(Prolog,
        ["tests-pl/issue-stateful-http_server", "-t", "run_uninterrupted"],
        [process(Process), stdout(pipe(Out))]).

server_wait_start([_Process, Out]) :-
    get_char(Out, _C).

server_stop([Process,_Out]) :-
    process_kill(Process).

% Client 
:- use_module(library(charsio)).
:- use_module(library(http/http_open)).

send_request(Path, Result) :-
    append("http://localhost:8472/", Path, URL),
    http_open(URL, Stream, []),
    get_line_to_chars(Stream,Result,"").

% client
val(Val) :- send_request("", Val).
inc(Val) :- send_request("inc", Val).
dec(Val) :- send_request("dec", Val).
times2(Val) :- send_request("times2", Val).
error(Val) :- send_request("error", Val).
missing(Val) :- send_request("missing", Val).
not_found(Val) :- send_request("not_found", Val).
echo(In,Resp) :- append("echo/", In, URL), send_request(URL, Resp).

run_tests([]).
run_tests([T|Ts]) :-
    T -> run_tests(Ts)
    ; write(["test failed", T]).

tests([
    val("0"),
    inc("1"),
    val("1"),
    inc("2"),
    times2("4"),
    dec("3"),
    error("Internal Server Error"),
    missing("Internal Server Error"),
    val("3"), % after errors, previous value is saved
    not_found("Not Found"),
    echo("hello", "hello"),
    val("3")
]).

main :-
    setup_call_cleanup(
        server_start(Server),
        ((
            server_wait_start(Server),
            tests(Tests),
            run_tests(Tests)
        )
        ;
        (
            write(fail),nl
        )),
        server_stop(Server)
    ).

:- initialization(main).
