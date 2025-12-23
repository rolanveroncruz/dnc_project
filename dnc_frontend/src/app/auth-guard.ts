import {inject} from '@angular/core';
import { CanActivateFn,Router } from '@angular/router';
import {LoginService} from './login.service';

export const authGuard: CanActivateFn = (route, state) => {
  const auth = inject(LoginService);
  const router = inject(Router);
  return auth.isLoggedIn()
    ? true: router.createUrlTree(['/login'], {
    queryParams: {returnUrl: state.url},
  })
};
