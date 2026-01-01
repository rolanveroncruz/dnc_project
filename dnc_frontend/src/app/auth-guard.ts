import {inject, PLATFORM_ID} from '@angular/core';
import { CanActivateFn,Router } from '@angular/router';
import {LoginService} from './login.service';
import {isPlatformBrowser} from '@angular/common';

export const authGuard: CanActivateFn = (route, state) => {
  const platformId = inject(PLATFORM_ID);

  if (!isPlatformBrowser(platformId)) {
    return true;
  }


  const auth = inject(LoginService);
  const router = inject(Router);
  return auth.isLoggedIn()
    ? true: router.createUrlTree(['/login'], {
    queryParams: {returnUrl: state.url},
  })
};
