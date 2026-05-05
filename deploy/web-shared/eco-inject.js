(function(){
  // shared theme across *.longevity.ge — cookie on parent domain, fallback to localStorage
  function getTheme(){
    var m = document.cookie.match(/(?:^|; )lc_theme=([^;]+)/);
    if (m) return decodeURIComponent(m[1]);
    return localStorage.getItem("theme");
  }
  function setTheme(v){
    var d = new Date(); d.setTime(d.getTime()+365*24*60*60*1000);
    document.cookie = "lc_theme="+encodeURIComponent(v)+"; expires="+d.toUTCString()+"; path=/; domain=.longevity.ge; SameSite=Lax";
    try { localStorage.setItem("theme", v); } catch(e) {}
  }

  var host = window.location.hostname;
  var path = window.location.pathname;

  // Annals (OJS) and Longevity Horizon (OJS) embed this script via the
  // theme plugin but must NOT inherit the home-style overrides — their
  // PKP layout has its own brand. Bail out before touching the page.
  if (path.indexOf("/rescience") === 0 || path.indexOf("/longhoriz") === 0) {
    return;
  }

  var t = getTheme();
  if (t === "dark") document.documentElement.setAttribute("data-theme","dark");
  var links = [
    ["https://longevity.ge","Home","longevity.ge","/"],
    ["https://mcoa.longevity.ge","MCOA","mcoa.longevity.ge",null],
    ["https://cdata.longevity.ge","CDATA","cdata.longevity.ge",null],
    ["https://ze.longevity.ge","Ze","ze.longevity.ge",null],
    ["https://biosense.longevity.ge","BioSense","biosense.longevity.ge",null],
    ["https://fclc.longevity.ge","FCLC","fclc.longevity.ge",null],
    ["https://hive.longevity.ge","Hive","hive.longevity.ge",null],
    ["https://aim.longevity.ge","AIM","aim.longevity.ge",null],
    ["https://longevity.ge/rescience/","Annals","longevity.ge","/rescience/"],
    ["https://longevity.ge/team/","Team","longevity.ge","/team/"],
    ["https://longevity.ge/#donate","Donate",null,null],
    ["https://github.com/djabbat/LongevityCommon","Source",null,null]
  ];
  function isActive(l){
    if (l[2] !== host) return false;
    if (l[3] === null) return true;
    if (l[3] === "/") {
      return path === "/" || path === "" || (path.indexOf("/rescience/") !== 0 && path.indexOf("/team/") !== 0 && path.indexOf("/longhoriz/") !== 0);
    }
    return path.indexOf(l[3]) === 0;
  }
  var nav = links.map(function(l){
    var cls = "";
    if (isActive(l)) cls = " class=\"active\"";
    if (l[1] === "Donate") cls = " class=\"donate-cta\"";
    var rel = (l[1] === "Source") ? " rel=\"noopener\"" : "";
    return "<a href=\"" + l[0] + "\"" + cls + rel + ">" + l[1] + "</a>";
  }).join("\n");

  var html = "<div class=\"eco-bar-injected\"><div class=\"eco-inner-i\"><span class=\"eco-brand-i\">LongevityCommon</span><nav class=\"eco-nav-i\">" + nav + "<button type=\"button\" class=\"theme-toggle-i\" aria-label=\"Toggle dark mode\">☾</button></nav></div></div>";

  var style = document.createElement("style");
  style.textContent = [
    ".eco-bar-injected{position:sticky !important;top:0 !important;z-index:100 !important;background:rgba(15,23,42,0.97) !important;backdrop-filter:blur(8px) !important;border-bottom:1px solid rgba(255,255,255,0.06) !important;font-family:Inter,-apple-system,system-ui,sans-serif !important;font-size:15px !important;line-height:1.4 !important;width:100% !important;box-sizing:border-box !important;margin:0 !important}",
    ".eco-inner-i{max-width:1100px !important;margin:0 auto !important;padding:12px 32px !important;display:flex !important;align-items:center !important;justify-content:space-between !important;gap:16px !important;flex-wrap:wrap !important;box-sizing:border-box !important;width:100% !important}",
    ".eco-brand-i{font-weight:700 !important;font-size:15px !important;color:#fff !important;letter-spacing:-0.01em !important;line-height:1.2 !important}",
    ".eco-brand-i::before{content:\"\\25CF\" !important;color:#4f46e5 !important;margin-right:8px !important;font-size:10px !important;vertical-align:middle !important}",
    ".eco-nav-i{display:flex !important;gap:2px !important;flex-wrap:wrap !important;align-items:center !important;font-size:13px !important;background:transparent !important;border:none !important;position:static !important}",
    ".eco-nav-i a{color:#cbd5e1 !important;padding:6px 12px !important;border-radius:6px !important;font-size:13px !important;font-weight:500 !important;transition:all 0.15s !important;text-decoration:none !important;line-height:1.2 !important;background:transparent !important;border:none !important}",
    ".eco-nav-i a:hover{background:rgba(255,255,255,0.08) !important;color:#fff !important}",
    ".eco-nav-i a.active{background:#4f46e5 !important;color:#fff !important}",
    ".eco-nav-i a.donate-cta{background:#dc2626 !important;color:#fff !important;font-weight:600 !important;padding:6px 14px !important;border-radius:6px !important;box-shadow:0 1px 3px rgba(220,38,38,0.4) !important}",
    ".eco-nav-i a.donate-cta:hover{background:#b91c1c !important;color:#fff !important;transform:translateY(-1px) !important}",
    ".eco-nav-i a.donate-cta::before{content:\"\\2665 \" !important;color:#fff !important;margin-right:2px !important}",
    ".theme-toggle-i{background:transparent !important;border:1px solid rgba(255,255,255,0.35) !important;color:#fff !important;cursor:pointer !important;padding:4px 10px !important;border-radius:4px !important;font-size:16px !important;margin-left:12px !important;line-height:1 !important}",
    ".theme-toggle-i:hover{background:rgba(255,255,255,0.12) !important}",
    "html[data-theme=\"dark\"] body{background:#0f1117 !important;color:#e0e3eb !important}",
    "html[data-theme=\"dark\"] .eco-bar-injected{background:rgba(6,8,15,0.97) !important}",
    "html[data-theme=\"dark\"] .header,html[data-theme=\"dark\"] .card,html[data-theme=\"dark\"] section,html[data-theme=\"dark\"] .axiom{background:#15171f !important;color:#d8dce4 !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] h1,html[data-theme=\"dark\"] h2,html[data-theme=\"dark\"] h3,html[data-theme=\"dark\"] h4{color:#fff !important}",
    "html[data-theme=\"dark\"] code,html[data-theme=\"dark\"] .formula,html[data-theme=\"dark\"] .badge{background:#2a2f40 !important;color:#e0e3eb !important}",
    "html[data-theme=\"dark\"] a{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .pkp_structure_page,html[data-theme=\"dark\"] .pkp_structure_head,html[data-theme=\"dark\"] .pkp_structure_main,html[data-theme=\"dark\"] .pkp_structure_content,html[data-theme=\"dark\"] .pkp_brand_footer,html[data-theme=\"dark\"] .pkp_footer_content,html[data-theme=\"dark\"] .pkp_block,html[data-theme=\"dark\"] .pkp_structure_footer,html[data-theme=\"dark\"] .pkp_structure_footer_wrapper{background:#0f1117 !important;color:#d8dce4 !important;border-color:#2a2f40 !important}",
"html[data-theme=\"dark\"] .pkp_site_name,html[data-theme=\"dark\"] .pkp_site_name *,html[data-theme=\"dark\"] .pkp_navigation_primary a,html[data-theme=\"dark\"] .pkp_navigation_user a,html[data-theme=\"dark\"] .pkp_brand_footer a{color:#fff !important}",
"html[data-theme=\"dark\"] .pkp_navigation_primary,html[data-theme=\"dark\"] .pkp_navigation_user,html[data-theme=\"dark\"] .pkp_navigation_primary_wrapper{background:#15171f !important;border-color:#2a2f40 !important}",
"html[data-theme=\"dark\"] .pkp_block,html[data-theme=\"dark\"] .pkp_block *{background-color:#15171f !important;color:#d8dce4 !important;border-color:#2a2f40 !important}",
"html[data-theme=\"dark\"] .pkp_structure_head{border-bottom-color:#2a2f40 !important}",
"html[data-theme=\"dark\"] input,html[data-theme=\"dark\"] textarea,html[data-theme=\"dark\"] select{background:#1a1d28 !important;color:#e0e3eb !important;border-color:#2a2f40 !important}",
"html[data-theme=\"dark\"] .pkp_search input{background:#1a1d28 !important;color:#e0e3eb !important}",
    "html[data-theme=\"dark\"] body,html[data-theme=\"dark\"] p,html[data-theme=\"dark\"] li,html[data-theme=\"dark\"] td,html[data-theme=\"dark\"] dt,html[data-theme=\"dark\"] dd,html[data-theme=\"dark\"] label,html[data-theme=\"dark\"] span,html[data-theme=\"dark\"] em,html[data-theme=\"dark\"] strong,html[data-theme=\"dark\"] small,html[data-theme=\"dark\"] article,html[data-theme=\"dark\"] section,html[data-theme=\"dark\"] div{color:#d8dce4}",
    "html[data-theme=\"dark\"] .eco-bar-injected,html[data-theme=\"dark\"] .eco-bar-injected *{color:inherit}",
    "html[data-theme=\"dark\"] .eco-nav-i a{color:#cbd5e1}",
    "html[data-theme=\"dark\"] .eco-brand-i{color:#fff}",
    "html[data-theme=\"dark\"] .pkp_block_title,html[data-theme=\"dark\"] .pkp_block li,html[data-theme=\"dark\"] .pkp_block a,html[data-theme=\"dark\"] .obj_announcement_summary,html[data-theme=\"dark\"] .obj_article_summary,html[data-theme=\"dark\"] .cmp_announcement_summary,html[data-theme=\"dark\"] .cmp_article_list,html[data-theme=\"dark\"] .pkp_structure_main *{color:#d8dce4}",
    "html[data-theme=\"dark\"] .pkp_structure_main h1,html[data-theme=\"dark\"] .pkp_structure_main h2,html[data-theme=\"dark\"] .pkp_structure_main h3,html[data-theme=\"dark\"] .pkp_structure_main h4{color:#fff}",
    "html[data-theme=\"dark\"] .pkp_structure_main a{color:#88a8ff}",
    "html[data-theme=\"dark\"] [style*=\"color:#18181b\"],html[data-theme=\"dark\"] [style*=\"color: #18181b\"],html[data-theme=\"dark\"] [style*=\"color:#27272a\"],html[data-theme=\"dark\"] [style*=\"color: #27272a\"],html[data-theme=\"dark\"] [style*=\"color:#3f3f46\"],html[data-theme=\"dark\"] [style*=\"color:#52525b\"],html[data-theme=\"dark\"] [style*=\"color: #52525b\"],html[data-theme=\"dark\"] [style*=\"color:#71717a\"],html[data-theme=\"dark\"] [style*=\"color: #71717a\"]{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] [style*=\"background:#fff\"],html[data-theme=\"dark\"] [style*=\"background: #fff\"],html[data-theme=\"dark\"] [style*=\"background:white\"],html[data-theme=\"dark\"] [style*=\"background:#fafafa\"],html[data-theme=\"dark\"] [style*=\"background:#f4f4f5\"]{background-color:#15171f !important}",
    "html[data-theme=\"dark\"] .obj_article_summary,html[data-theme=\"dark\"] .obj_issue_toc,html[data-theme=\"dark\"] .cmp_article_list,html[data-theme=\"dark\"] .current_issue,html[data-theme=\"dark\"] .homepage_about,html[data-theme=\"dark\"] .highlights,html[data-theme=\"dark\"] .footer-container,html[data-theme=\"dark\"] .swiper-slide,html[data-theme=\"dark\"] .swiper-slide-content{background-color:#15171f !important;color:#d8dce4 !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .obj_article_summary .title,html[data-theme=\"dark\"] .obj_article_summary .title a,html[data-theme=\"dark\"] .current_issue_title,html[data-theme=\"dark\"] .swiper-slide-title,html[data-theme=\"dark\"] .journal-name,html[data-theme=\"dark\"] .section,html[data-theme=\"dark\"] .sections,html[data-theme=\"dark\"] h2.title,html[data-theme=\"dark\"] h3.title{color:#fff !important}",
    "html[data-theme=\"dark\"] .obj_article_summary .authors,html[data-theme=\"dark\"] .authors,html[data-theme=\"dark\"] .meta,html[data-theme=\"dark\"] .meta *,html[data-theme=\"dark\"] .description,html[data-theme=\"dark\"] .published,html[data-theme=\"dark\"] .label,html[data-theme=\"dark\"] .heading,html[data-theme=\"dark\"] .issn,html[data-theme=\"dark\"] .copyright,html[data-theme=\"dark\"] .rights-access{color:#c8ccd5 !important}",
    "html[data-theme=\"dark\"] .obj_galley_link,html[data-theme=\"dark\"] .obj_galley_link.pdf,html[data-theme=\"dark\"] .read_more,html[data-theme=\"dark\"] .pkp_button,html[data-theme=\"dark\"] .swiper-slide-button{background-color:#1a2440 !important;color:#fff !important;border-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .obj_galley_link:hover,html[data-theme=\"dark\"] .pkp_button:hover{background-color:#2a3450 !important}",
    "html[data-theme=\"dark\"] .swiper-pagination-bullet{background:#88a8ff !important}",
    "html[data-theme=\"dark\"] .galleys_links{background:transparent !important}",
    "html[data-theme=\"dark\"] a:not(.pkp_button):not(.obj_galley_link):not(.read_more){color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .page_article,html[data-theme=\"dark\"] .page_article *,html[data-theme=\"dark\"] .main_entry,html[data-theme=\"dark\"] .main_entry *,html[data-theme=\"dark\"] .entry_details,html[data-theme=\"dark\"] .entry_details *,html[data-theme=\"dark\"] .obj_article_details,html[data-theme=\"dark\"] .obj_article_details *{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .page_article h1,html[data-theme=\"dark\"] .page_title,html[data-theme=\"dark\"] .label,html[data-theme=\"dark\"] .item h2,html[data-theme=\"dark\"] .item h3,html[data-theme=\"dark\"] .obj_article_details .label{color:#fff !important}",
    "html[data-theme=\"dark\"] .csl-bib-body,html[data-theme=\"dark\"] .csl-entry,html[data-theme=\"dark\"] .csl-bib-body *,html[data-theme=\"dark\"] .csl-entry *,html[data-theme=\"dark\"] .references,html[data-theme=\"dark\"] .references *,html[data-theme=\"dark\"] .item.references,html[data-theme=\"dark\"] .item.references *{color:#c8ccd5 !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .item.abstract,html[data-theme=\"dark\"] .item.abstract *,html[data-theme=\"dark\"] .item.authors,html[data-theme=\"dark\"] .item.authors *,html[data-theme=\"dark\"] .item.published,html[data-theme=\"dark\"] .item.published *,html[data-theme=\"dark\"] .item.issue,html[data-theme=\"dark\"] .item.section,html[data-theme=\"dark\"] .item.keywords,html[data-theme=\"dark\"] .item.copyright,html[data-theme=\"dark\"] .item.doi,html[data-theme=\"dark\"] .item.citation,html[data-theme=\"dark\"] .item.galleys{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .userGroup,html[data-theme=\"dark\"] .profile,html[data-theme=\"dark\"] .name,html[data-theme=\"dark\"] .citation_formats_styles,html[data-theme=\"dark\"] .sub_item,html[data-theme=\"dark\"] .citation_display,html[data-theme=\"dark\"] .cmp_breadcrumbs,html[data-theme=\"dark\"] .cmp_breadcrumbs *,html[data-theme=\"dark\"] .newsletter-signup-ojs,html[data-theme=\"dark\"] .newsletter-signup-ojs *,html[data-theme=\"dark\"] .pflPlugin,html[data-theme=\"dark\"] .pflPlugin *{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .page_article a,html[data-theme=\"dark\"] .csl-entry a,html[data-theme=\"dark\"] .references a,html[data-theme=\"dark\"] .citation_formats_styles a{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .citation_formats_list{background-color:#15171f !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .citation_formats_button{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .getftr,html[data-theme=\"dark\"] .getftr *{background:transparent !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .getftr [class*=\"button\"],html[data-theme=\"dark\"] .getftr [role=\"button\"]{background:#1a2440 !important;color:#fff !important;border-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .newsletter-signup-ojs{background:#15171f !important;border-left-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .newsletter-signup-ojs *{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .newsletter-signup-ojs input[type=email]{background:#1a1d28 !important;color:#e0e3eb !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .newsletter-signup-ojs input[type=email]::placeholder{color:#7a808d !important}",
    "html[data-theme=\"dark\"] .newsletter-signup-ojs button[type=submit]{background:#1a2440 !important;color:#fff !important;border-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .citation_display,html[data-theme=\"dark\"] #citationOutput,html[data-theme=\"dark\"] .citation_display .value,html[data-theme=\"dark\"] .citation_display .label,html[data-theme=\"dark\"] .item.citation,html[data-theme=\"dark\"] .item.citation *,html[data-theme=\"dark\"] .csl-bib-body,html[data-theme=\"dark\"] .csl-entry,html[data-theme=\"dark\"] .csl-entry i,html[data-theme=\"dark\"] .csl-entry em{background:transparent !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .citation_display .label{color:#fff !important}",
    "html[data-theme=\"dark\"] .csl-entry a,html[data-theme=\"dark\"] .item.citation a{color:#88a8ff !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .item.doi,html[data-theme=\"dark\"] .item.doi *{background:transparent !important}",
    "html[data-theme=\"dark\"] .item.doi .value a{color:#88a8ff !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .pkp_block,html[data-theme=\"dark\"] .pkp_block .content,html[data-theme=\"dark\"] .pkp_block .content ul,html[data-theme=\"dark\"] .pkp_block .content li{background:#15171f !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .pkp_block .title{color:#fff !important;border-bottom-color:#88a8ff !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .obj_galley_link.pdf{background:#1a2440 !important;color:#fff !important;border:1px solid #88a8ff !important}",
    "html[data-theme=\"dark\"] aside[style*=\"background\"],html[data-theme=\"dark\"] section[style*=\"background\"],html[data-theme=\"dark\"] div[style*=\"background:#f\"],html[data-theme=\"dark\"] div[style*=\"background: #f\"]{background:#15171f !important}",
    "html[data-theme=\"dark\"] [style*=\"color:#0a2540\"],html[data-theme=\"dark\"] [style*=\"color: #0a2540\"],html[data-theme=\"dark\"] [style*=\"color:#222\"],html[data-theme=\"dark\"] [style*=\"color: #222\"]{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] header:not(.eco-bar-injected),html[data-theme=\"dark\"] footer{background:#15171f !important;color:#d8dce4 !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] header:not(.eco-bar-injected) *,html[data-theme=\"dark\"] footer *{color:#d8dce4 !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] header:not(.eco-bar-injected) a,html[data-theme=\"dark\"] footer a{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .bg-white,html[data-theme=\"dark\"] .bg-zinc-50,html[data-theme=\"dark\"] .bg-zinc-100,html[data-theme=\"dark\"] .bg-zinc-200,html[data-theme=\"dark\"] .bg-gray-50,html[data-theme=\"dark\"] .bg-gray-100{background-color:#15171f !important}",
    "html[data-theme=\"dark\"] .text-zinc-500,html[data-theme=\"dark\"] .text-zinc-600,html[data-theme=\"dark\"] .text-zinc-700,html[data-theme=\"dark\"] .text-zinc-800,html[data-theme=\"dark\"] .text-zinc-900,html[data-theme=\"dark\"] .text-gray-500,html[data-theme=\"dark\"] .text-gray-600,html[data-theme=\"dark\"] .text-gray-700,html[data-theme=\"dark\"] .text-gray-800,html[data-theme=\"dark\"] .text-gray-900{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .border-zinc-100,html[data-theme=\"dark\"] .border-zinc-200,html[data-theme=\"dark\"] .border-zinc-300,html[data-theme=\"dark\"] .border-gray-100,html[data-theme=\"dark\"] .border-gray-200,html[data-theme=\"dark\"] .border-gray-300{border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .hover\\:bg-zinc-100:hover,html[data-theme=\"dark\"] .hover\\:bg-zinc-200:hover,html[data-theme=\"dark\"] .hover\\:text-zinc-700:hover,html[data-theme=\"dark\"] .hover\\:text-zinc-900:hover{background-color:#1a2440 !important;color:#fff !important}",
    "html[data-theme=\"dark\"] .bg-blue-600{background-color:#4f46e5 !important}",
    "header:not(.eco-bar-injected) > *,footer > *,.max-w-7xl,.max-w-6xl,.max-w-5xl,.max-w-4xl{max-width:1100px !important;margin-left:auto !important;margin-right:auto !important;box-sizing:border-box !important}",
    ".container,.header-inner,.footer-inner,.section-inner,.page-hero-inner,.hero-inner,.footer-grid,main > section,main > article,main > div{max-width:1100px !important;margin-left:auto !important;margin-right:auto !important;box-sizing:border-box !important}",
    ".section .title,.pkp_block .title,.issue_heading,.issue_identify,.pkp_navigation_primary ul,.pkp_structure_footer,h2.pkp_helpers_align_left{border-top-color:transparent !important;border-bottom-color:transparent !important;border-left-color:transparent !important}",
    "[style*=\"crimson\"]{border-color:transparent !important}",
    ".card[style*=\"crimson\"],.card[style*=\"border-top:3px solid var(--crimson)\"],.card[style*=\"border-top: 3px solid var(--crimson)\"]{border-top:0 !important}",
    "html[data-theme=\"dark\"] .pkp_navigation_primary,html[data-theme=\"dark\"] .pkp_navigation_primary_wrapper,html[data-theme=\"dark\"] .pkp_navigation_primary_row,html[data-theme=\"dark\"] .pkp_navigation_user_wrapper,html[data-theme=\"dark\"] .pkp_navigation_search_wrapper,html[data-theme=\"dark\"] .pkp_navigation_user{background:#15171f !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .pkp_navigation_primary a,html[data-theme=\"dark\"] .pkp_navigation_user a,html[data-theme=\"dark\"] .pkp_navigation_primary ul a{color:#e0e3eb !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .pkp_navigation_primary a:hover,html[data-theme=\"dark\"] .pkp_navigation_primary li.current > a,html[data-theme=\"dark\"] .pkp_navigation_user a:hover,html[data-theme=\"dark\"] .pkp_navigation_primary ul a:hover{color:#88a8ff !important;background:#1a2440 !important}",
    "html[data-theme=\"dark\"] .pkp_navigation_primary ul{background:#15171f !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .pkp_navigation_primary ul a{border-bottom-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .pkp_search{color:#e0e3eb !important;background:#1a1d28 !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .pkp_search:hover{color:#88a8ff !important;border-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .pkp_search input,html[data-theme=\"dark\"] .pkp_navigation_search_wrapper input{background:#1a1d28 !important;color:#e0e3eb !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .pkp_site_name a{color:#fff !important}",
    "html[data-theme=\"dark\"] .pkp_site_name a:hover{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .dropdown-menu{background:#15171f !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .dropdown-menu a,html[data-theme=\"dark\"] .dropdown-menu li a{color:#e0e3eb !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .dropdown-menu a:hover{color:#88a8ff !important;background:#1a2440 !important}",
    "html[data-theme=\"dark\"] .pkp_site_nav_menu,html[data-theme=\"dark\"] .pkp_navigation_primary_row{background:#15171f !important}",
    "html[data-theme=\"dark\"] .footer-container,html[data-theme=\"dark\"] .footer-container *{background-color:#15171f !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .footer-container a{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .homepage_about,html[data-theme=\"dark\"] .homepage_about *{background:#15171f !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .obj_article_summary{background:#15171f !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .item.galleys,html[data-theme=\"dark\"] .page_article .item.galleys{background:#15171f !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] fieldset,html[data-theme=\"dark\"] .page_search fieldset,html[data-theme=\"dark\"] fieldset.search_advanced,html[data-theme=\"dark\"] .page_search fieldset.search_advanced{background:#15171f !important;border-color:#2a2f40 !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] fieldset legend,html[data-theme=\"dark\"] .page_search fieldset legend{color:#fff !important;background:transparent !important}",
    "html[data-theme=\"dark\"] fieldset label,html[data-theme=\"dark\"] fieldset .label,html[data-theme=\"dark\"] .page_search label{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] select,html[data-theme=\"dark\"] option{background:#1a1d28 !important;color:#e0e3eb !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] input[type=text],html[data-theme=\"dark\"] input[type=search],html[data-theme=\"dark\"] input[type=email],html[data-theme=\"dark\"] input[type=password],html[data-theme=\"dark\"] input[type=number]{background:#1a1d28 !important;color:#e0e3eb !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] input::placeholder,html[data-theme=\"dark\"] textarea::placeholder{color:#7a808d !important}",
    "html[data-theme=\"dark\"] .page_search input.query,html[data-theme=\"dark\"] .page_search .search_input input[type=text]{background:#1a1d28 !important;color:#e0e3eb !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .galleys_links,html[data-theme=\"dark\"] .galleys_links li{background:transparent !important;list-style:none !important}",
    "html[data-theme=\"dark\"] .galleys_links li a,html[data-theme=\"dark\"] .obj_galley_link,html[data-theme=\"dark\"] .obj_galley_link.pdf,html[data-theme=\"dark\"] .obj_galley_link:not(.pdf){background:#1a2440 !important;color:#fff !important;border:1px solid #88a8ff !important}",
    "html[data-theme=\"dark\"] .galleys_links li a:hover,html[data-theme=\"dark\"] .obj_galley_link:hover,html[data-theme=\"dark\"] .obj_galley_link.pdf:hover{background:#2a3450 !important;color:#fff !important;border-color:#a8bfff !important}",
    "html[data-theme=\"dark\"] .swiper-slide-content,html[data-theme=\"dark\"] .swiper-slide-desc,html[data-theme=\"dark\"] .swiper-slide-title{color:#fff !important}",
    "html[data-theme=\"dark\"] .swiper-slide-button.pkp_button{background:#1a2440 !important;color:#fff !important;border-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .pkp_brand_footer,html[data-theme=\"dark\"] .pkp_brand_footer *{background:#0f1117 !important;color:#d8dce4 !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .pkp_brand_footer a{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .cmp_breadcrumbs,html[data-theme=\"dark\"] .cmp_breadcrumbs li,html[data-theme=\"dark\"] .cmp_breadcrumbs a{color:#d8dce4 !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .cmp_breadcrumbs a{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .heading,html[data-theme=\"dark\"] .heading *{background:transparent !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .pkp_structure_head,html[data-theme=\"dark\"] .pkp_head_wrapper,html[data-theme=\"dark\"] .pkp_site_name_wrapper{background:#15171f !important;border-color:#2a2f40 !important;box-shadow:none !important}",
    "html[data-theme=\"dark\"] .pkp_site_name a,html[data-theme=\"dark\"] .longevity-journal-name{color:#fff !important}",
    "html[data-theme=\"dark\"] .longevity-platform-corner,html[data-theme=\"dark\"] .longevity-platform-corner:hover,html[data-theme=\"dark\"] .longevity-platform-corner:visited,html[data-theme=\"dark\"] .longevity-platform-corner:focus{color:#88a8ff !important;opacity:0.7 !important}",
    "html[data-theme=\"dark\"] .pkp_site_nav_toggle{color:#e0e3eb !important;background:transparent !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .task_count{background:#1a2440 !important;color:#88a8ff !important;border-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .pkp_navigation_primary li.current{background:transparent !important}",
    "html[data-theme=\"dark\"] .longevity-platform-title{color:#fff !important}",
    "html[data-theme=\"dark\"] .section > h2,html[data-theme=\"dark\"] .section > h3,html[data-theme=\"dark\"] .sections .section > h2,html[data-theme=\"dark\"] .sections .section > h3{background:#15171f !important;color:#fff !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .section{border-top-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .section::before,html[data-theme=\"dark\"] .section::after{border-color:#2a2f40 !important}",
    /* Project essence panel — collapsible info block injected on Phoenix subdomains */
    ".lc-essence{max-width:1100px !important;margin:18px auto !important;padding:0 24px !important;font-family:Inter,-apple-system,system-ui,sans-serif !important;display:block !important;width:100% !important;box-sizing:border-box !important}",
    ".lc-essence-toggle{width:100% !important;display:flex !important;align-items:center !important;justify-content:space-between !important;gap:12px !important;background:rgba(79,70,229,0.08) !important;border:1px solid rgba(79,70,229,0.25) !important;border-radius:6px !important;padding:12px 16px !important;cursor:pointer !important;font-size:15px !important;font-weight:600 !important;color:#1e1b4b !important;font-family:inherit !important;text-align:left !important;line-height:1.3 !important}",
    ".lc-essence-toggle:hover{background:rgba(79,70,229,0.14) !important;border-color:rgba(79,70,229,0.45) !important}",
    ".lc-essence-title{flex:1 !important;color:#1e1b4b !important}",
    ".lc-essence-chev{color:#4f46e5 !important;font-size:14px !important}",
    ".lc-essence-body{margin-top:12px !important;padding:18px 22px !important;background:#fff !important;border:1px solid #e2e8f0 !important;border-left:3px solid #4f46e5 !important;border-radius:0 6px 6px 0 !important;font-size:15px !important;line-height:1.6 !important;color:#1f2937 !important}",
    ".lc-essence-body p{margin:0 0 12px !important}",
    ".lc-essence-body p:last-child{margin-bottom:0 !important}",
    ".lc-essence-body code{background:#f1f5f9 !important;padding:1px 6px !important;border-radius:3px !important;font-family:ui-monospace,Menlo,monospace !important;font-size:13px !important;color:#1e293b !important}",
    ".lc-essence-body a{color:#4f46e5 !important;text-decoration:underline !important}",
    "html[data-theme=\"dark\"] .lc-essence-toggle{background:rgba(99,102,241,0.18) !important;border-color:rgba(129,140,248,0.4) !important;color:#e0e7ff !important}",
    "html[data-theme=\"dark\"] .lc-essence-toggle:hover{background:rgba(99,102,241,0.28) !important;border-color:rgba(129,140,248,0.6) !important}",
    "html[data-theme=\"dark\"] .lc-essence-title{color:#e0e7ff !important}",
    "html[data-theme=\"dark\"] .lc-essence-chev{color:#a5b4fc !important}",
    "html[data-theme=\"dark\"] .lc-essence-body{background:#15171f !important;border-color:#2a2f40 !important;border-left-color:#818cf8 !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .lc-essence-body code{background:#2a2f40 !important;color:#e0e3eb !important}",
    "html[data-theme=\"dark\"] .lc-essence-body a{color:#a5b4fc !important}",

    /* ── HOME-PALETTE BASE (matches longevity.ge root index.html) ──
     * Applied to every *.longevity.ge page that loads eco-inject.js
     * EXCEPT /rescience/ (Annals — OJS not allowed to inherit).
     * Uses generous specificity (html prefix + !important) to win over
     * subdomain-specific stylesheets while leaving page content alone.
     */
    "html:not([data-theme=\"dark\"]):root{--c-text:#0f172a !important;--c-text-soft:#475569 !important;--c-text-muted:#64748b !important;--c-bg:#f8fafc !important;--c-card:#ffffff !important;--c-border:#e2e8f0 !important;--c-border-strong:#cbd5e1 !important;--c-accent:#4f46e5 !important;--c-accent-soft:#eef2ff !important;--c-success:#10b981 !important;--c-warning:#f59e0b !important;--c-danger:#ef4444 !important;--radius:12px;--radius-lg:16px}",
    "html:not([data-theme=\"dark\"]) body{font-family:Inter,-apple-system,BlinkMacSystemFont,system-ui,sans-serif !important;background:#f8fafc;color:#0f172a;line-height:1.6;-webkit-font-smoothing:antialiased;-moz-osx-font-smoothing:grayscale}",
    "html:not([data-theme=\"dark\"]) code,html:not([data-theme=\"dark\"]) .formula,html:not([data-theme=\"dark\"]) pre{font-family:'JetBrains Mono',ui-monospace,Menlo,monospace !important}",
    "html:not([data-theme=\"dark\"]) h1,html:not([data-theme=\"dark\"]) h2,html:not([data-theme=\"dark\"]) h3,html:not([data-theme=\"dark\"]) h4{color:#0f172a;letter-spacing:-0.02em}",
    "html:not([data-theme=\"dark\"]) a{color:#4f46e5;transition:color 0.15s}",
    "html:not([data-theme=\"dark\"]) a:hover{color:#6366f1}",

    /* ── Common component primitives (apply to BOTH light & dark) ──
     * Using prefix `html` (not `:root`) so every subdomain — Phoenix,
     * static landings, FastAPI templates — picks up the same look.
     * !important wins over subdomain-specific styles, but we leave
     * page-internal selectors (`.simulator`, `.results`, …) alone.
     */
    "html .container,html .hero-inner,html .footer-inner,html .page-inner{max-width:1100px !important;margin-left:auto !important;margin-right:auto !important;box-sizing:border-box !important}",
    "html .container{padding:4.5rem 2rem !important}",
    "html .hero{background:linear-gradient(135deg,#1e1b4b 0%,#312e81 35%,#4338ca 75%,#6366f1 100%) !important;color:#fff !important;padding:4rem 2.5rem !important;position:relative !important;overflow:hidden !important}",
    "html .hero::before{content:'' !important;position:absolute !important;top:0 !important;right:0 !important;width:60% !important;height:100% !important;background:radial-gradient(circle at top right,rgba(167,139,250,0.25),transparent 60%) !important;pointer-events:none !important}",
    "html .hero h1,html .hero .hero-title{font-size:clamp(2.5rem,5vw,3.75rem) !important;font-weight:800 !important;line-height:1.05 !important;margin:0 0 1rem 0 !important;letter-spacing:-0.025em !important;color:#fff !important}",
    "html .hero p,html .hero .hero-subtitle{font-size:clamp(1rem,1.5vw,1.1875rem) !important;line-height:1.55 !important;opacity:0.92 !important;max-width:48rem !important;margin:0 0 1.75rem 0 !important;color:#fff !important}",
    "html .hero-pill{display:inline-flex !important;align-items:center !important;gap:0.5rem !important;background:rgba(255,255,255,0.08) !important;border:1px solid rgba(255,255,255,0.18) !important;padding:0.4375rem 1rem !important;border-radius:999px !important;font-size:0.75rem !important;font-weight:600 !important;letter-spacing:0.08em !important;text-transform:uppercase !important;margin-bottom:1.25rem !important;color:#fff !important}",
    "html .hero-pill::before{content:'' !important;width:6px !important;height:6px !important;border-radius:50% !important;background:#34d399 !important;box-shadow:0 0 8px #34d399 !important}",
    "html .btn{display:inline-flex !important;align-items:center !important;gap:0.5rem !important;padding:0.75rem 1.375rem !important;border-radius:999px !important;font-weight:600 !important;font-size:0.9375rem !important;text-decoration:none !important;transition:transform 0.15s,box-shadow 0.15s !important;cursor:pointer !important;font-family:inherit !important;border:none !important}",
    "html .btn-primary{background:#fff !important;color:#312e81 !important;box-shadow:0 4px 12px rgba(0,0,0,0.12) !important}",
    "html .btn-primary:hover{transform:translateY(-1px) !important;box-shadow:0 6px 20px rgba(0,0,0,0.18) !important;color:#312e81 !important}",
    "html .btn-ghost{background:transparent !important;color:#fff !important;border:1.5px solid rgba(255,255,255,0.4) !important}",
    "html .btn-ghost:hover{background:rgba(255,255,255,0.10) !important}",
    "html .section-title{font-size:1.75rem !important;font-weight:700 !important;letter-spacing:-0.015em !important;margin:3rem 0 0.5rem 0 !important}",
    "html .section-lead{color:#475569 !important;font-size:1rem !important;margin:0 0 1.5rem 0 !important;max-width:48rem !important}",
    "html:not([data-theme=\"dark\"]) .grid{display:grid !important;grid-template-columns:repeat(auto-fit,minmax(20rem,1fr)) !important;gap:1rem !important}",
    "html:not([data-theme=\"dark\"]) .card{background:#fff !important;border:1px solid #e2e8f0 !important;border-radius:16px !important;padding:1.5rem !important;transition:transform 0.18s,box-shadow 0.18s,border-color 0.18s !important}",
    "html:not([data-theme=\"dark\"]) .card.link{cursor:pointer !important}",
    "html:not([data-theme=\"dark\"]) .card.link:hover{transform:translateY(-3px) !important;box-shadow:0 4px 12px rgba(15,23,42,0.06) !important;border-color:#4f46e5 !important;text-decoration:none !important}",
    "html:not([data-theme=\"dark\"]) .card h3{margin:0 0 0.625rem 0 !important;font-size:1.1875rem !important;font-weight:700 !important;color:#0f172a !important}",
    "html:not([data-theme=\"dark\"]) .card p{margin:0.625rem 0 !important;font-size:0.9375rem !important;color:#475569 !important;line-height:1.55 !important}",
    "html:not([data-theme=\"dark\"]) .card .role{font-size:0.6875rem !important;color:#64748b !important;text-transform:uppercase !important;letter-spacing:0.08em !important;font-weight:600 !important;margin-bottom:0.5rem !important}",
    "html:not([data-theme=\"dark\"]) .badge{display:inline-flex !important;font-size:0.6875rem !important;font-weight:600 !important;padding:0.1875rem 0.625rem !important;border-radius:999px !important;letter-spacing:0.02em !important}",
    "html:not([data-theme=\"dark\"]) .badge.green{background:#d1fae5 !important;color:#065f46 !important}",
    "html:not([data-theme=\"dark\"]) .badge.blue{background:#dbeafe !important;color:#1e40af !important}",
    "html:not([data-theme=\"dark\"]) .badge.purple{background:#ede9fe !important;color:#5b21b6 !important}",
    "html:not([data-theme=\"dark\"]) .badge.gray{background:#f1f5f9 !important;color:#475569 !important}",
    "html:not([data-theme=\"dark\"]) .badge.warn{background:#fef3c7 !important;color:#92400e !important}",
    "html:not([data-theme=\"dark\"]) .badge.red{background:#fee2e2 !important;color:#991b1b !important}",
    /* ── Subdomain-internal headers normalised to home-style ─────── */
    "html:not([data-theme=\"dark\"]) header:not(.eco-bar-injected),html:not([data-theme=\"dark\"]) .site-header,html:not([data-theme=\"dark\"]) .aim-subnav{background:#fff !important;border-bottom:1px solid #e2e8f0 !important;color:#0f172a !important;font-family:Inter,sans-serif !important}",
    "html:not([data-theme=\"dark\"]) header:not(.eco-bar-injected) a,html:not([data-theme=\"dark\"]) .site-header a,html:not([data-theme=\"dark\"]) .aim-subnav a{color:#475569 !important}",
    "html:not([data-theme=\"dark\"]) header:not(.eco-bar-injected) a:hover,html:not([data-theme=\"dark\"]) .site-header a:hover,html:not([data-theme=\"dark\"]) .aim-subnav a:hover{color:#4f46e5 !important;background:#eef2ff !important;text-decoration:none !important}",
    "html:not([data-theme=\"dark\"]) header:not(.eco-bar-injected) a.active,html:not([data-theme=\"dark\"]) .aim-subnav a.active{background:#4f46e5 !important;color:#fff !important}",
    /* ── Footer normalised ──────────────────────────────────────── */
    "html:not([data-theme=\"dark\"]) footer{background:#fff !important;border-top:1px solid #e2e8f0 !important;color:#64748b !important;font-family:Inter,sans-serif !important}",
    "html:not([data-theme=\"dark\"]) footer a{color:#4f46e5 !important}"
  ].join("\n");

  // Inject Inter + JetBrains Mono fonts (idempotent — skip if already present)
  if (!document.querySelector('link[href*="fonts.googleapis.com/css2?family=Inter"]')) {
    var fp = document.createElement("link");
    fp.rel = "preconnect";
    fp.href = "https://fonts.googleapis.com";
    document.head.appendChild(fp);
    var fp2 = document.createElement("link");
    fp2.rel = "preconnect";
    fp2.href = "https://fonts.gstatic.com";
    fp2.crossOrigin = "anonymous";
    document.head.appendChild(fp2);
    var ff = document.createElement("link");
    ff.rel = "stylesheet";
    ff.href = "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500&display=swap";
    document.head.appendChild(ff);
  }

  // Favicon — one emoji per subdomain. Idempotent: skip if a non-default
  // <link rel="icon"> is already present in <head>.
  function ensureFavicon(){
    var faviconMap = {
      "mcoa.longevity.ge":      "\u{1F9EE}",  // abacus — multi-counter
      "cdata.longevity.ge":     "\u{1F52C}",  // microscope — centriolar damage
      "ze.longevity.ge":        "\u{1F300}",  // cyclone — Ze entropic-geometric
      "biosense.longevity.ge":  "\u{1F4E1}",  // satellite — wearable sensor
      "fclc.longevity.ge":      "\u{1F517}",  // chain — federated
      "hive.longevity.ge":      "\u{1F41D}",  // bee — Hive (already in queen HTML)
      "longevity.ge":           "\u{1F331}"   // seedling — root
    };
    var emoji = faviconMap[host];
    if (!emoji) return;
    // If any <link rel~="icon"> already present and non-default, do nothing.
    var existing = document.querySelector('link[rel~="icon"]');
    if (existing && existing.getAttribute("href") &&
        existing.getAttribute("href").indexOf("favicon.ico") === -1) {
      return;
    }
    if (existing) existing.parentNode.removeChild(existing);
    var svg = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">' +
              '<text y=".9em" font-size="90">' + emoji + '</text></svg>';
    var link = document.createElement("link");
    link.rel = "icon";
    link.type = "image/svg+xml";
    link.href = "data:image/svg+xml;utf8," + encodeURIComponent(svg);
    document.head.appendChild(link);
  }

  // Project essence injection was removed 2026-05-04 — Phoenix templates now
  // ship the essence block directly (see Ze SimulatorLive, BioSense
  // SimulatorLive, FCLC PageLive). Static landings (MCOA, CDATA, Hive)
  // carry their own essence in-page. Function kept as a no-op for
  // graceful degradation if a stale page references it.
  function injectEssence(){ return; /* superseded by template-level content */
    /* legacy stub:
    var essences = {
      "ze.longevity.ge": {
        title: "Ze Theory · entropic-geometric ansatz",
        body:
          "<p><strong>The interactive widget on this page</strong> simulates the central law of Ze Theory: <code>dτ_Ze/dt = −α · I(Z)</code>, where <code>I(Z)</code> is the Kullback-Leibler divergence between actual and modelled state. From this single ansatz the simulator <em>derives mathematically</em> a quadratic CHSH deformation, the LGI-QFI bound (Abboud 2026), and the universal fixed point <code>v* = 0.45631</code> at <code>k_λ = 1</code>.</p>" +
          "<p><strong>Why it exists.</strong> The aging field has decoupled \"information\" (epigenetic clocks, biomarkers) from \"thermodynamics\" (entropy production, dissipation). Ze Theory unifies them via a single quantity: prediction error. A system that predicts itself well burns less time; a system whose model decays burns more. The χ_Ze fixed point is what falls out of the variational principle <code>F = E − T·S − λ·I_pred</code>.</p>" +
          "<p><strong>Status.</strong> Internal manuscript, not peer-reviewed (Tkemaladze 2026, <em>Longevity Horizon</em> 2(5), DOI <a href=\"https://doi.org/10.65649/xf5vp867\">10.65649/xf5vp867</a>). Mathematical derivations passing CI; biological extension is hypothesis-stage — BioSense empirically confirms the v* fixed point on AoU N=500.</p>" +
          "<p><strong>How to use.</strong> Drag <code>k_λ</code>, <code>δ</code>, and <code>i</code> sliders below to watch the CHSH deformation and decay curves update in real time. The simulator runs entirely server-side; no data is sent except the slider values. Read the <a href=\"/about\">/about</a> page for the full derivation and references.</p>" +
          "<p><strong>For:</strong> theorists · physicists checking the CHSH/LGI/QFI derivation · readers cross-validating the v* fixed point.</p>"
      },
      "biosense.longevity.ge": {
        title: "BioSense · wearable χ_Ze biomarker",
        body:
          "<p><strong>The dashboard on this page</strong> reads sample EEG / HRV / respiration / sleep traces and computes the χ_Ze aging-activity biomarker continuously. Variational principle: <code>F = E − T·S − λ·I_pred</code>. Theoretical fixed point <code>v* = 0.45631</code>; sensitivity range <code>[0.32, 0.58]</code> for <code>k_λ ∈ [0.5, 2.0]</code>.</p>" +
          "<p><strong>Empirical validation (2026).</strong> Swept-v* search on All-of-Us N=500 returned <code>v*_optimal = 0.451 (95% CI 0.443–0.459)</code> — consistent with the theoretical prediction. Confirmatory pre-registered N≥2000 trial pending EIC funding.</p>" +
          "<p><strong>Honest disclosure.</strong> The multimodal weights (0.30, 0.30, 0.20, 0.20) for EEG · HRV · respiration · sleep are <em>post-hoc</em> pilot fits, not theory-fixed. They will be re-derived under the pre-registered protocol before the confirmatory trial.</p>" +
          "<p><strong>Privacy.</strong> Raw signals never leave the device; only the scalar χ_Ze is exported. The on-device estimator is open-source (Python/NumPy reference, mobile WebUI). For federated cohort studies see <a href=\"https://fclc.longevity.ge\">FCLC</a>.</p>" +
          "<p><strong>For:</strong> wearable-device engineers · sleep scientists · exacerbation-prediction clinicians · AoU/UK Biobank reusers.</p>"
      },
      "fclc.longevity.ge": {
        title: "FCLC · federated clinical learning cooperative",
        body:
          "<p><strong>The orchestrator dashboard on this page</strong> shows live federation rounds, ε spent (Renyi-DP accountant), Krum-rejected updates, and the contribution leaderboard. Each participating clinic deploys a local node; raw patient data never leaves the clinic.</p>" +
          "<p><strong>Privacy stack.</strong> Renyi differential privacy (Mironov 2017, ε ≤ 1.0/round, ε_total ≈ 0.43 at σ=1.5, q=0.013, T=5), k-anonymity (k ≥ 7), Krum Byzantine-robust aggregation (tolerates ≤ 25% malicious clients), SecAgg+ secure aggregation (Bonawitz 2017 + Shamir secret sharing). v13.4 PASS milestone reached on all unit tests.</p>" +
          "<p><strong>Threat model (explicit).</strong> Secure ONLY against semi-honest server + Byzantine clients. NOT secure against active server collusion or a malicious server. <strong>GDPR Article 9 blocker</strong> until FCLC v14 (active-adversary migration, planned Q1 2027).</p>" +
          "<p><strong>Role in the ecosystem.</strong> FCLC is the privacy-preserving infrastructure that lets MCOA counter-weight w_i(tissue) be calibrated across multi-site cohorts without raw data transfer. Without FCLC, MCOA cannot reach the N≥2000 falsification cohort required by Axiom M4.</p>" +
          "<p><strong>For:</strong> hospital IT · GDPR / DPO officers · clinical AI engineers wanting to participate in MCOA validation · federation researchers studying SecAgg/RDP composition.</p>"
      }
    };
    var e = essences[host];
    if (!e) return;
    var KEY = "lc_essence_" + host;
    var collapsed = localStorage.getItem(KEY) === "1";
    var wrap = document.createElement("section");
    wrap.className = "lc-essence";
    wrap.setAttribute("aria-label", "Project essence");
    wrap.innerHTML =
      '<button type="button" class="lc-essence-toggle" aria-expanded="' + (!collapsed) + '">' +
        '<span class="lc-essence-title">ℹ ' + e.title + '</span>' +
        '<span class="lc-essence-chev">' + (collapsed ? "▸" : "▾") + '</span>' +
      '</button>' +
      '<div class="lc-essence-body" ' + (collapsed ? 'hidden' : '') + '>' + e.body + '</div>';
    // Insert after eco-bar (which is body's first child by now).
    var bar = document.querySelector(".eco-bar-injected");
    if (bar && bar.nextSibling) {
      document.body.insertBefore(wrap, bar.nextSibling);
    } else {
      document.body.appendChild(wrap);
    }
    var btn = wrap.querySelector(".lc-essence-toggle");
    var bodyEl = wrap.querySelector(".lc-essence-body");
    var chev = wrap.querySelector(".lc-essence-chev");
    btn.addEventListener("click", function(){
      var nowCollapsed = !bodyEl.hidden;
      bodyEl.hidden = nowCollapsed;
      btn.setAttribute("aria-expanded", String(!nowCollapsed));
      chev.textContent = nowCollapsed ? "▸" : "▾";
      localStorage.setItem(KEY, nowCollapsed ? "1" : "0");
    });
    */
  }

  function init(){
    document.head.appendChild(style);
    ensureFavicon();
    // Idempotent: a Phoenix template (e.g. AIM) may already have rendered
    // <div class="eco-bar-injected"> server-side. Don't add a second one.
    if (document.querySelector(".eco-bar-injected")) {
      injectEssence();
      return;
    }
    var div = document.createElement("div");
    div.innerHTML = html;
    document.body.insertBefore(div.firstChild, document.body.firstChild);
    injectEssence();
    var btn = document.querySelector(".theme-toggle-i");
    function syncIcon(){
      var dark = document.documentElement.getAttribute("data-theme") === "dark";
      btn.textContent = dark ? "☀" : "☾";
    }
    btn.addEventListener("click", function(){
      var dark = document.documentElement.getAttribute("data-theme") === "dark";
      if (dark) {
        document.documentElement.removeAttribute("data-theme");
        setTheme("light");
      } else {
        document.documentElement.setAttribute("data-theme","dark");
        setTheme("dark");
      }
      syncIcon();
    });
    syncIcon();
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
