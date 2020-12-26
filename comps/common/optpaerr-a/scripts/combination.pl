#!/usr/bin/perl
#
# sudo apt install libalgorithm-combinatorics-perl
#
use strict;
use Algorithm::Combinatorics qw(combinations);

my $max_cnt = $#ARGV + 1;

for (my $k=1; $k<$max_cnt; $k+=1) {
  my $iter = combinations(\@ARGV, $k);
  while (my $p = $iter->next) {
    print @$p,"\n";
  }
}
