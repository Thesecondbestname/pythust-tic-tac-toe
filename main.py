Game.start_ask(for_starting_player))
        .until(player_is_valid)
        .repeating_by(asking_again)
        .then(print_the_board)
        .after_that_ask(for_player(input()))
            .until(that_input_is_valid)
            .repeating_by(asking_again)
            .then(update_the_board)
            .and(print_the_board)
            .until(someone_won)
    .finally_ask(if_they_want_again)
    .until(they_dont_anymore)


    
.start_ask(beginning_player(input()))) # Until {action_result, State}
    .until(input_is_valid_player) # RepeatUntil {stack, predicate(action_result) -> Option<T>, State}
    .repeating_by(asking_again) # for loop
