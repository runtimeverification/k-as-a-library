module ARITHMETIC-SYNTAX
  imports UNSIGNED-INT-SYNTAX

  syntax Exp ::= Int
endmodule

module ARITHMETIC
  imports ARITHMETIC-SYNTAX

  configuration
    <k> $PGM:Exp </k>
endmodule
