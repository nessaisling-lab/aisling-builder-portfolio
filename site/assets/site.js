// Progressive enhancement only. Every page works with this file blocked.
(function () {

  // Links people already have (an email, a résumé) point at the old one-page site's
  // anchors, or at the About page that folded into Home. Send them where it lives now.
  var moved = {
    '#projects': 'work/', '#founder': 'work/nisaba/', '#experience': 'experience/',
    '#documents': 'resume/', '#contact': 'contact/', '#about': ''
  };
  if (document.querySelector('.room') && moved[location.hash] !== undefined) {
    location.replace(moved[location.hash] || './');
    return;
  }
  var aboutMoved = { '#experience': 'experience/', '#education': 'school/', '#stack': 'experience/#stack', '#contact': 'contact/' };
  if (document.querySelector('.moved')) {
    location.replace('../' + (aboutMoved[location.hash] || ''));
    return;
  }

  // Reading settings (text size, spacing, contrast, day or evening, motion, finish), from
  // the brand kit. Without the kit's script the slot stays empty and hidden.
  var slot = document.querySelector('.settings-slot');
  if (slot && window.NessBrand) NessBrand.settings.mount(slot);

  // Projects: pressing a bay raises that build onto the lift. Without this, each bay is a
  // plain link to its build; a modified click (new tab, new window) still opens the link.
  var lift = document.querySelector('.lift');
  if (lift) {
    var bays = document.querySelectorAll('.wall .nb-plate');
    var raise = function (slug, focus) {
      var panel = slug && document.getElementById('lift-' + slug);
      if (!panel || !panel.classList.contains('raised')) return false;
      lift.querySelectorAll('.raised').forEach(function (p) { p.hidden = p !== panel; });
      bays.forEach(function (a) { a.setAttribute('aria-current', a.dataset.slug === slug ? 'true' : 'false'); });
      if (focus) panel.querySelector('h2').focus();
      return true;
    };
    bays.forEach(function (a) {
      a.dataset.slug = (a.getAttribute('href').match(/work\/([^/]+)\/$/) || [])[1];
      a.setAttribute('aria-controls', 'lift-' + a.dataset.slug);
      a.addEventListener('click', function (e) {
        if (e.button || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
        if (raise(a.dataset.slug, true)) { e.preventDefault(); history.replaceState(null, '', '#' + a.dataset.slug); }
      });
    });
    var up = lift.querySelector('.raised:not([hidden])');
    if (!raise(location.hash.slice(1), false) && up) raise(up.id.slice(5), false);
  }

  // Build recordings play while on screen, unless the reader prefers less motion.
  // Without this script they are ordinary videos with controls.
  var still = matchMedia('(prefers-reduced-motion: reduce)').matches;
  var vids = document.querySelectorAll('video[data-autoplay]');
  if (!still && 'IntersectionObserver' in window && vids.length) {
    var io = new IntersectionObserver(function (entries) {
      entries.forEach(function (e) {
        if (e.isIntersecting) { e.target.preload = 'auto'; e.target.play().catch(function () {}); }
        else { e.target.pause(); }
      });
    }, { threshold: 0.4 });
    vids.forEach(function (v) { io.observe(v); });
  }

  // Phones: fold role, team and stack so the video comes first. Without JS they stay open.
  var more = document.querySelectorAll('details.spec-more');
  if (more.length) {
    var narrow = matchMedia('(max-width: 52rem)');
    var fold = function () { more.forEach(function (d) { d.open = !narrow.matches; }); };
    fold();
    narrow.addEventListener('change', fold);
  }

  // Contact form: send in place and say what happened. Without JS it posts normally.
  var form = document.getElementById('contactForm');
  if (form) {
    var status = document.getElementById('formStatus');
    form.addEventListener('submit', function (e) {
      e.preventDefault();
      var send = form.querySelector('button[type=submit]');
      send.disabled = true;
      status.className = 'form-status';
      status.textContent = 'Sending…';
      fetch(form.action, { method: 'POST', body: new FormData(form), headers: { Accept: 'application/json' } })
        .then(function (r) { return r.json(); })
        .then(function (d) {
          if (d.success) { status.textContent = 'Sent. I will reply by email.'; form.reset(); }
          else { status.className = 'form-status err'; status.textContent = (d.message || 'That did not send.') + ' Email ness.aisling@gmail.com instead.'; }
        })
        .catch(function () {
          status.className = 'form-status err';
          status.textContent = 'That did not send. Email ness.aisling@gmail.com instead.';
        })
        .then(function () { send.disabled = false; });
    });
  }
})();
