pub fn print_init_script(shell_type: &str) {
    if shell_type == "zsh" {
        let script = r#"
    setopt PROMPT_SUBST
    zmodload zsh/net/socket 2>/dev/null
    
    starex_connect() {
        if [[ -n $STAREX_READ_FD ]]; then
            zle -F $STAREX_READ_FD 2>/dev/null
            exec {STAREX_READ_FD}>&- 2>/dev/null
        fi
        zsocket /tmp/starex.sock 2>/dev/null
        if [[ -n $REPLY ]]; then
            export STAREX_READ_FD=$REPLY
            zle -F $STAREX_READ_FD handle_starex_update
        fi
    }
    
    handle_starex_update() {
        local data latest
    
        if ! read -u $1 data; then
            starex_connect
            return
        fi
        latest=$data
    
        while read -u $1 -t 0 data; do
            read -u $1 latest
        done
    
        export STAREX_PROMPT_VAL="$latest"
        zle reset-prompt 2>/dev/null
    }
    
    starex_connect
    
    starex_send_input() {
        zle .self-insert 2>/dev/null
        
        if [[ -S /tmp/starex.sock ]]; then
            ( 
                zsocket /tmp/starex.sock 2>/dev/null && {
                    print -u $REPLY "BUFFER:$BUFFER"
                }
            ) &!
        fi
    }
    
    zle -N self-insert starex_send_input
    "#;
        println!("{}", script);
    } else if shell_type == "bash" {
        println!("{}", "TODO!");
    } else if shell_type == "fish" {
        println!("{}", "TODO!");
    }
}
