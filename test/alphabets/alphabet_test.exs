defmodule ExBio.Alphabets.AlphabetTest do
  use ExUnit.Case
  doctest ExBio
  alias ExBio.Alphabets.Alphabet

  describe "new" do
    test "create new empty alphabet" do
      {:ok, alphabet} = Alphabet.new("")
      assert is_reference(alphabet) == true
    end

    test "create new non-empty alphabet" do
      {:ok, alphabet} = Alphabet.new("AACCTgga")
      assert is_reference(alphabet) == true
    end
  end

  describe "insert and len" do
    test "add new symbols to the empty alphabet" do
      {:ok, alphabet} = Alphabet.new("")
      len1 = Alphabet.len(alphabet)
      {:ok, _} = Alphabet.insert(alphabet, ?A)
      {:ok, _} = Alphabet.insert(alphabet, ?C)
      len2 = Alphabet.len(alphabet)

      assert len1 == 0
      assert len2 == 2
    end

    test "add new symbol to the alphabet" do
      {:ok, alphabet} = Alphabet.new("AACCTgg")
      len1 = Alphabet.len(alphabet)
      {:ok, _} = Alphabet.insert(alphabet, ?a)
      len2 = Alphabet.len(alphabet)

      assert len1 == 4
      assert len2 == 5
    end
  end

  describe "is_word" do
    test "false on empty alphabet" do
      {:ok, alphabet} = Alphabet.new("")
      is_word = Alphabet.is_word(alphabet, "AACCTgg")

      assert is_word == false
    end

    test "false when not a word" do
      {:ok, alphabet} = Alphabet.new("AACCT")
      is_word = Alphabet.is_word(alphabet, "AACCTgg")

      assert is_word == false
    end

    test "true when a word" do
      {:ok, alphabet} = Alphabet.new("AACCTgg")
      is_word = Alphabet.is_word(alphabet, "AACCT")

      assert is_word == true
    end
  end

  describe "max_symbol" do
    test "nil when empty alphabet" do
      {:ok, alphabet} = Alphabet.new("")
      {:ok, max_symbol} = Alphabet.max_symbol(alphabet)

      assert max_symbol == nil
    end

    test "return the max symbol" do
      {:ok, alphabet} = Alphabet.new("AACCTgg")
      {:ok, max_symbol} = Alphabet.max_symbol(alphabet)

      assert max_symbol == 103
    end
  end

  describe "is_empty" do
    test "true when empty alphabet" do
      {:ok, alphabet} = Alphabet.new("")
      assert Alphabet.is_empty(alphabet) == true
    end

    test "false when non-empty alphabet" do
      {:ok, alphabet} = Alphabet.new("AACCT")
      assert Alphabet.is_empty(alphabet) == false
    end

    test "false after adding symbol to empty alphabet" do
      {:ok, alphabet} = Alphabet.new("")
      Alphabet.insert(alphabet, ?A)
      assert Alphabet.is_empty(alphabet) == false
    end
  end
end
