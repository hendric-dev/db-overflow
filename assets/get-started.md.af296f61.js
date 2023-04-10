import{_ as s,c as a,o as n,N as l}from"./chunks/framework.64f7bd5a.js";const d=JSON.parse('{"title":"Get Started","description":"","frontmatter":{"title":"Get Started"},"headers":[],"relativePath":"get-started.md"}'),e={name:"get-started.md"},o=l(`<h1 id="get-started" tabindex="-1">Get Started <a class="header-anchor" href="#get-started" aria-label="Permalink to &quot;Get Started&quot;">​</a></h1><p>DB Overflow is designed as a command line interface. It&#39;s as easy as downloading the prebuilt binary and run it from a terminal.</p><h2 id="download" tabindex="-1">Download <a class="header-anchor" href="#download" aria-label="Permalink to &quot;Download&quot;">​</a></h2><p>Binaries can be downloaded from the <a href="https://github.com/hendric-dev/db-overflow/releases" target="_blank" rel="noreferrer">Releases</a> page.</p><p>An example using curl:</p><div class="language-sh"><button title="Copy Code" class="copy"></button><span class="lang">sh</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#A6ACCD;">platform</span><span style="color:#89DDFF;">=</span><span style="color:#C3E88D;">linux-amd64</span></span>
<span class="line"><span style="color:#A6ACCD;">version</span><span style="color:#89DDFF;">=</span><span style="color:#C3E88D;">0.1.0</span></span>
<span class="line"><span style="color:#FFCB6B;">curl</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">-L</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">https://github.com/hendric-dev/db-overflow/releases/download/</span><span style="color:#A6ACCD;">$version</span><span style="color:#C3E88D;">/db-overflow-</span><span style="color:#A6ACCD;">$platform </span><span style="color:#C3E88D;">-o</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">db-overflow</span></span>
<span class="line"></span></code></pre></div><p>Make it executable and it is ready to be used:</p><div class="language-sh"><button title="Copy Code" class="copy"></button><span class="lang">sh</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#FFCB6B;">chmod</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">a+x</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">db-overflow</span></span>
<span class="line"></span></code></pre></div><h2 id="usage" tabindex="-1">Usage <a class="header-anchor" href="#usage" aria-label="Permalink to &quot;Usage&quot;">​</a></h2><p>The tool is self documenting, just run it with the <code>--help</code> flag to get an overview:</p><div class="language-sh"><button title="Copy Code" class="copy"></button><span class="lang">sh</span><pre class="shiki material-theme-palenight"><code><span class="line"><span style="color:#FFCB6B;">$</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">db-overflow</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">--help</span></span>
<span class="line"></span>
<span class="line"><span style="color:#FFCB6B;">DB</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">Overflow</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">-</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">Insert</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">large</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">amounts</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">of</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">data</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">into</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">a</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">Postgres</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">DB</span></span>
<span class="line"></span>
<span class="line"><span style="color:#FFCB6B;">Usage:</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">db-overflow</span><span style="color:#A6ACCD;"> </span><span style="color:#89DDFF;">&lt;</span><span style="color:#C3E88D;">COMMAN</span><span style="color:#A6ACCD;">D</span><span style="color:#89DDFF;">&gt;</span></span>
<span class="line"></span>
<span class="line"><span style="color:#FFCB6B;">Commands:</span></span>
<span class="line"><span style="color:#A6ACCD;">  </span><span style="color:#FFCB6B;">config</span><span style="color:#A6ACCD;">  </span><span style="color:#C3E88D;">Creates</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">a</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">config</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">file</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">from</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">a</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">DB</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">table</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">for</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">more</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">fine</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">grained</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">customizations</span></span>
<span class="line"><span style="color:#A6ACCD;">  </span><span style="color:#FFCB6B;">run</span><span style="color:#A6ACCD;">     </span><span style="color:#C3E88D;">Generate</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">data</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">and</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">fill</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">your</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">DB</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">table</span></span>
<span class="line"><span style="color:#A6ACCD;">  </span><span style="color:#FFCB6B;">help</span><span style="color:#A6ACCD;">    </span><span style="color:#C3E88D;">Print</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">this</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">message</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">or</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">the</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">help</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">of</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">the</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">given</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">subcommand</span><span style="color:#89DDFF;">(</span><span style="color:#FFCB6B;">s</span><span style="color:#89DDFF;">)</span></span>
<span class="line"></span>
<span class="line"><span style="color:#FFCB6B;">Options:</span></span>
<span class="line"><span style="color:#A6ACCD;">  </span><span style="color:#FFCB6B;">-h,</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">--help</span><span style="color:#A6ACCD;">     </span><span style="color:#C3E88D;">Print</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">help</span></span>
<span class="line"><span style="color:#A6ACCD;">  </span><span style="color:#FFCB6B;">-V,</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">--version</span><span style="color:#A6ACCD;">  </span><span style="color:#C3E88D;">Print</span><span style="color:#A6ACCD;"> </span><span style="color:#C3E88D;">version</span></span>
<span class="line"></span></code></pre></div><p>Options can either be passed as CLI arguments or are loaded from environment variables.</p><h2 id="build" tabindex="-1">Build <a class="header-anchor" href="#build" aria-label="Permalink to &quot;Build&quot;">​</a></h2><p>It is also possible to build the tool from scratch if needed.</p><h3 id="prerequisites" tabindex="-1">Prerequisites <a class="header-anchor" href="#prerequisites" aria-label="Permalink to &quot;Prerequisites&quot;">​</a></h3><ul><li><a href="https://www.rust-lang.org/tools/install" target="_blank" rel="noreferrer">Rust</a></li></ul><h3 id="usage-1" tabindex="-1">Usage <a class="header-anchor" href="#usage-1" aria-label="Permalink to &quot;Usage&quot;">​</a></h3><p>Use <code>cargo run -- --help</code> to compile and run the program. <br> Otherwise the usage is exactly the same as the binary.</p><div class="tip custom-block"><p class="custom-block-title">TIP</p><p>There are some default values set in the <code>.env</code> file of the project, which are loaded automatically.</p></div><h3 id="release" tabindex="-1">Release <a class="header-anchor" href="#release" aria-label="Permalink to &quot;Release&quot;">​</a></h3><p>To compile a release binary use <code>cargo build --release</code>. <br> An optimized binary is created inside the <code>target</code> folder.</p>`,21),p=[o];function t(r,c,C,i,y,D){return n(),a("div",null,p)}const h=s(e,[["render",t]]);export{d as __pageData,h as default};