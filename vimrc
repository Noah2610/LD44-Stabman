nmap <leader>rr :!RUN_NEW_TERMINAL=1 bin/run-dev<CR>
nmap <leader>rR :!RUN_NEW_TERMINAL=1 bin/run-dev --release<CR>
nmap <leader>rb :!RUN_NEW_TERMINAL=1 bin/build-dev<CR>
nmap <leader>rB :!RUN_NEW_TERMINAL=1 bin/build-dev --release<CR>
nmap <leader>rt :!RUN_NEW_TERMINAL=1 RUST_BACKTRACE=1 bin/run-dev 2>&1<CR>
nmap <leader>rT :!RUN_NEW_TERMINAL=1 RUST_BACKTRACE=1 bin/run-dev --release 2>&1<CR>
