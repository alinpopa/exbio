defmodule ExBio.PatternMatching.BomTest do
  use ExUnit.Case
  doctest ExBio

  test "bom" do
    assert ExBio.PatternMatching.Bom.bom("ACGGCTAGGAAAAAGACTGAGGACTGAAAA", "GAAAA") ==
             {:ok, [8, 25]}
  end
end
