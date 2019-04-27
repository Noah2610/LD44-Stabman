nmap <leader>r :!bin/run<CR>
nmap <leader>R :!RUST_BACKTRACE=1 bin/run 2>&1 \| grep -EA 1 'LD44\|deathframe'<CR>
