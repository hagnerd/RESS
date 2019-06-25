#git checkout benchable_master && \
#cargo +nightly bench

STEP="master"
for filename in ./target/criterion/*/new/raw.csv; do
    ONE="${filename/\.\/target\/criterion\/}"
    BENCH="${ONE/\/new\/raw.csv/}"
    DEST="./criterion/$STEP/$BENCH.csv"
    cp ${filename} ${DEST}
done
#git checkout next && \
#cargo +nightly bench

#STEP="next"
#for filename in ./target/criterion/*/new/raw.csv; do
#    ONE="${filename/\.\/target\/criterion\/}"
#    BENCH="${ONE/\/new\/raw.csv/}"
#    DEST="./criterion/$STEP/$BENCH.csv"
#    cp ${filename} ${DEST}
#done
node ./perf.js > master_vs_next.md
