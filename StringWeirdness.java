public class StringWeirdness {
	public static void main(String[] args) {
		// these strings look the same...

		// but this string has a precomposed `é` (`U+00E9`)
		String s = "tést";
		// and this string has a regular `e` (`U+0065`) followed by a combining mark (`U+0301`)
		String t = "tést";
		System.out.printf("s = \"%s\"\n", s);
		System.out.printf("s's length is %s\n", s.length());  // 4
		System.out.printf("character 1 of s is %s\n\n", s.charAt(1)); // é
		System.out.printf("t = \"%s\"\n", t);
		System.out.printf("t's length is %s\n", t.length());  // 5??
		System.out.printf("character 1 of t is %s\n\n", t.charAt(1)); // e??


		// Emoji are even weirder, because they have very large codepoints beyond the
		// original 16-bit range of Unicode, so they are represented as 2 Java "char"s.
		// So, a single "char" variable cannot contain an emoji. Oops!
		String e = "😀";

		System.out.printf("e = \"%s\"\n", e);
		System.out.printf("e's length is %s\n", e.length());  // 2
		System.out.printf("character 0 of e is U+%04X\n", (int)e.charAt(0));
		System.out.printf("character 1 of e is U+%04X\n", (int)e.charAt(1));

		// codePointAt combines the two 16-bit characters at 0 and 1 into a 21-bit codepoint.
		System.out.printf("codepoint 0 of e is U+%06X\n", e.codePointAt(0));
	}
}