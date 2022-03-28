sz=(3 5 10 30 50 80 100 150 500)

for s in ${sz[@]}
do 
  echo doing $s
  RUST_LOG=info cargo run --  synthesize example my-net --initial-variant $s
  mv time_log logs/time_$s.log
done  
