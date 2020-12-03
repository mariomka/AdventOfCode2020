use re 'eval';

open my $input, '../day2/input.txt' or die "Could not open file";

my $counter = 0;

while(my $line = <$input>)  {
    if ($line =~ /^(\d+)-(\d+) ([a-z]): ((??{ "((?:(?!$3)\[a-z\])*$3(?:(?!$3)\[a-z\])*)\{$1,$2\}" }))$/) {
        $counter++;
    }
}

close $input;

print "${counter}\n";
