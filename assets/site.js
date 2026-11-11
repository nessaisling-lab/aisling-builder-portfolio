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
