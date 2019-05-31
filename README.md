# commonmark spec vs pulldown_cmark

To examine the compability of [pulldown_cmark](https://github.com/raphlinus/pulldown-cmark) to [the commonmark specification](https://spec.commonmark.org/)  

commonmark spec's test cases are published [here](https://spec.commonmark.org/)  

# versions
- 2019/05/31
- pulldown-cmark = "0.5.2"
- [Commonmark 0.29](https://spec.commonmark.org/0.29/)

# Result

```
total: 649 specs examined, error: 9 specs detected.
```


```
Error: Suite {
    markdown: "  Foo *bar\nbaz*\t\n====\n",
    html: "<h1>Foo <em>bar\nbaz</em></h1>\n",
    start_line: 1034,
    end_line: 1041,
    section: "Setext headings",
}
Expected output:
<h1>Foo <em>bar
baz</em></h1>

Actual output:
<h1>Foo <em>bar
baz</em>	</h1>

-----------------
Error: Suite {
    markdown: " <div>\n  *hello*\n         <foo><a>\n",
    html: " <div>\n  *hello*\n         <foo><a>\n",
    start_line: 2138,
    end_line: 2146,
    section: "HTML blocks",
}
Expected output:
 <div>
  *hello*
         <foo><a>

Actual output:
<div>
  *hello*
         <foo><a>

-----------------
Error: Suite {
    markdown: "- <div>\n- foo\n",
    html: "<ul>\n<li>\n<div>\n</li>\n<li>foo</li>\n</ul>\n",
    start_line: 2480,
    end_line: 2490,
    section: "HTML blocks",
}
Expected output:
<ul>
<li>
<div>
</li>
<li>foo</li>
</ul>

Actual output:
<ul>
<li><div>
</li>
<li>foo</li>
</ul>

-----------------
Error: Suite {
    markdown: "  <!-- foo -->\n\n    <!-- foo -->\n",
    html: "  <!-- foo -->\n<pre><code>&lt;!-- foo --&gt;\n</code></pre>\n",
    start_line: 2608,
    end_line: 2616,
    section: "HTML blocks",
}
Expected output:
  <!-- foo -->
<pre><code>&lt;!-- foo --&gt;
</code></pre>

Actual output:
<!-- foo -->
<pre><code>&lt;!-- foo --&gt;
</code></pre>

-----------------
Error: Suite {
    markdown: "  <div>\n\n    <div>\n",
    html: "  <div>\n<pre><code>&lt;div&gt;\n</code></pre>\n",
    start_line: 2619,
    end_line: 2627,
    section: "HTML blocks",
}
Expected output:
  <div>
<pre><code>&lt;div&gt;
</code></pre>

Actual output:
<div>
<pre><code>&lt;div&gt;
</code></pre>

-----------------
Error: Suite {
    markdown: "<table>\n\n  <tr>\n\n    <td>\n      Hi\n    </td>\n\n  </tr>\n\n</table>\n",
    html: "<table>\n  <tr>\n<pre><code>&lt;td&gt;\n  Hi\n&lt;/td&gt;\n</code></pre>\n  </tr>\n</table>\n",
    start_line: 2768,
    end_line: 2789,
    section: "HTML blocks",
}
Expected output:
<table>
  <tr>
<pre><code>&lt;td&gt;
  Hi
&lt;/td&gt;
</code></pre>
  </tr>
</table>

Actual output:
<table>
<tr>
<pre><code>&lt;td&gt;
  Hi
&lt;/td&gt;
</code></pre>
</tr>
</table>

-----------------
Error: Suite {
    markdown: "[foo] \n[]\n\n[foo]: /url \"title\"\n",
    html: "<p><a href=\"/url\" title=\"title\">foo</a>\n[]</p>\n",
    start_line: 8292,
    end_line: 8300,
    section: "Links",
}
Expected output:
<p><a href="/url" title="title">foo</a>
[]</p>

Actual output:
<p><a href="/url" title="title">foo</a>
[]</p>

-----------------
Error: Suite {
    markdown: "![foo] \n[]\n\n[foo]: /url \"title\"\n",
    html: "<p><img src=\"/url\" alt=\"foo\" title=\"title\" />\n[]</p>\n",
    start_line: 8620,
    end_line: 8628,
    section: "Images",
}
Expected output:
<p><img src="/url" alt="foo" title="title" />
[]</p>

Actual output:
<p><img src="/url" alt="foo" title="title" />
[]</p>

-----------------
Error: Suite {
    markdown: "foo \n baz\n",
    html: "<p>foo\nbaz</p>\n",
    start_line: 9329,
    end_line: 9335,
    section: "Soft line breaks",
}
Expected output:
<p>foo
baz</p>

Actual output:
<p>foo
baz</p>

-----------------
total: 649 specs examined, error: 9 specs detected.
```
