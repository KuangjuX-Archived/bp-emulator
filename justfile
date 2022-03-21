m1 		     	     := "6"
bimodal_file 	     := "traces/perl_trace.txt"
bimodal_output_file  := "bimodal_4.txt"

m2 		     	     := "10"
n            		 := "6"
gshare_file  	     := "traces/perl_trace.txt"
gshare_output_file   := "gshare_4.txt"

bimodal:
    cargo run --bin bimodal {{m1}} {{bimodal_file}} {{bimodal_output_file}}

gshare: 
    cargo run --bin gshare {{m2}} {{n}} {{gshare_file}} {{gshare_output_file}}

clean:
    cargo clean
    rm *.txt