define_state_group!(script_data_states_group = {

    pub script_data_state {
        // TODO
        eof => ( emit_eof; )
        _   => ( emit_eof; )
    }

});
