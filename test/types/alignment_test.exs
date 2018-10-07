defmodule ExBio.Types.AlignmentTest do
  use ExUnit.Case
  doctest ExBio

  describe "new" do
    test "creates reference" do
      alignment = %ExBio.Types.Alignment{operations: [{:xclip, 123}], mode: :local}
      {:ok, alignment_ref} = ExBio.Types.Alignment.new(alignment)
      assert is_reference(alignment_ref)
    end

    test "raises when passing invalid data" do
      assert_raise ArgumentError, fn ->
        ExBio.Types.Alignment.new(%{wrong: :struct})
      end
    end

    test "returns invalid args when passing wrong alignment data" do
      alignment = %ExBio.Types.Alignment{operations: [{:xclip, 123}], mode: nil}
      assert ExBio.Types.Alignment.new(alignment) == {:error, :invalid_args}
    end
  end
end
