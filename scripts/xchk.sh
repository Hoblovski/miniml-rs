#!/bin/bash
#
# cross checking miniml-rs with miniml.
#
# run as ./scripts/xchk.sh
#

MINIML=miniml
RSMINIML=target/debug/miniml
RSSECDI=target/debug/secdi
SECDI=secdi

TESTCASES=$(cd testcases; ls -1 *.ml)

if ! mkdir outs; then
	echo "outs already exists, delete (y/N)?"
	read -r cmd
	case $cmd in
		'y')
			rm -rf outs
			mkdir outs
			;;
		*)
			exit 1
			;;
	esac
fi

do_case() {
	local tc=$1
	printf "%-50s" $tc
	if ! ( $MINIML testcases/$tc > outs/$tc.1.secd 2>&1 ); then
		echo "MINIML failed"
		return
	fi
	if ! ( $RSMINIML testcases/$tc > outs/$tc.2.secd 2>&1 ); then
		echo "RSMINIML failed"
		return
	fi
	$RSSECDI outs/$tc.1.secd -b > outs/$tc.1.out 2>&1
	$RSSECDI outs/$tc.2.secd -b > outs/$tc.2.out 2>&1
	if diff outs/$tc.1.out outs/$tc.2.out -sq >/dev/null 2>&1; then
		echo ok
	else
		echo exec result differ
	fi
}

for tc in ${TESTCASES[@]}; do
	do_case $tc
done
