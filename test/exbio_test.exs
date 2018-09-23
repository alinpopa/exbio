defmodule ExBioTest do
  use ExUnit.Case
  doctest ExBio

  test "pattern matching bom" do
    assert ExBio.PatternMatching.Bom.bom("ACGGCTAGGAAAAAGACTGAGGACTGAAAA", "GAAAA") ==
             {:ok, [8, 25]}
  end
end
