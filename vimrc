nmap <leader>rr :!RUN_NEW_TERMINAL=1 RUN_DEV=1 RUN_FEATURES=nightly,controller,debug bin/run<CR>
nmap <leader>rR :!RUN_NEW_TERMINAL=1 RUN_DEV=1 RUN_FEATURES=nightly,controller,debug bin/run --release<CR>
nmap <leader>rb :!RUN_NEW_TERMINAL=1 RUN_FEATURES=nightly,controller,debug bin/build<CR>
nmap <leader>rB :!RUN_NEW_TERMINAL=1 RUN_FEATURES=nightly,controller,debug bin/build --release<CR>

nmap <leader>rt :!bin/tas/tas<CR>
