import { SfeirThemeInitializer } from '../web_modules/sfeir-school-theme/sfeir-school-theme.mjs';

// One method per module
function schoolSlides() {
  return ['02-codelab-leptos/00-TITLE.md', 'speakers/ly.md','02-codelab-leptos/01-BASICS.md', '02-codelab-leptos/02-EXERCISE.md', '02-codelab-leptos/00-TITLE.md'];
}

function introSlides() {
  return ['intro/00-TITLE.md'];
}

function formation() {
  return [
    //
    ...schoolSlides(), //
    ...introSlides(), //
  ].map((slidePath) => {
    return { path: slidePath };
  });
}

SfeirThemeInitializer.init(formation);
