module counter(out, clk, rst);
  output reg [3:0] out;
  input            clk;
  input            rst;

  always @(posedge clk)
      out <= rst ? out + 1 : 0;

endmodule

