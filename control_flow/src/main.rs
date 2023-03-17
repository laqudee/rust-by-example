use control_flow::if_else;
use control_flow::loop_play;
use control_flow::while_play;
use control_flow::for_play;
use control_flow::match_play;
use control_flow::if_let_play;
use control_flow::while_let_play;

fn main() {
    if_else::play();
    loop_play::play();
    loop_play::label_loop_play();
    loop_play::loop_break_value();
    while_play::play();
    for_play::play();
    for_play::for_iter_play();
    for_play::for_itermut_play();
    match_play::play();
    match_play::tuple_play();
    match_play::enum_play();
    match_play::ref_play();
    match_play::struct_play();
    match_play::guard_play();
    match_play::bingding_play();
    if_let_play::play();
    while_let_play::play();
}
