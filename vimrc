nmap <leader>rr :!RUN_NEW_TERMINAL=1 bin/run<CR>
nmap <leader>rR :!RUN_NEW_TERMINAL=1 bin/run --release<CR>
nmap <leader>rb :!RUN_NEW_TERMINAL=1 bin/build<CR>
nmap <leader>rB :!RUN_NEW_TERMINAL=1 bin/build --release<CR>
nmap <leader>rt :!RUN_NEW_TERMINAL=1 RUST_BACKTRACE=1 bin/run 2>&1<CR>
nmap <leader>rT :!RUN_NEW_TERMINAL=1 RUST_BACKTRACE=1 bin/run --release 2>&1<CR>
