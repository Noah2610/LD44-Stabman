nmap <leader>rr :!bin/run<CR>
nmap <leader>rR :!bin/run --release<CR>
nmap <leader>rb :!bin/build<CR>
nmap <leader>rB :!bin/build --release<CR>
nmap <leader>rt :!RUST_BACKTRACE=1 bin/run 2>&1 \| grep -EA 1 'LD44\|deathframe'<CR>
