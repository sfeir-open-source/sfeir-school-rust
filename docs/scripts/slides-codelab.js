import { SfeirThemeInitializer } from '../web_modules/sfeir-school-theme/sfeir-school-theme.mjs';

// One method per module
function schoolSlides() {
  return ['01-codelab/00-TITLE.md', 'speakers/ly.md','01-codelab/01-BASICS.md', '01-codelab/02-EXERCISE.md'];
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
