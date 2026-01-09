import {Component, signal} from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import {MatIconModule} from "@angular/material/icon";
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import {LoginService} from "../login.service";
import {firstValueFrom} from 'rxjs';
import {NavigationError, Router} from "@angular/router";
import {filter} from 'rxjs/operators';
import {HttpErrorResponse} from '@angular/common/http';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [
    MatCardModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    MatIconModule,
    FormsModule,
    ReactiveFormsModule,
  ],
  templateUrl: './login.html',
  styleUrl: './login.scss',
})
export class LoginComponent {
  email: string = "";
  password: string = "";
  showPassword: boolean = false;

  isLoading = signal(false);
  errorMessage = signal<string |null>(null);

  constructor(private LoginService: LoginService, private router: Router) {
    router.events
      .pipe(filter(e => e instanceof NavigationError))
      .subscribe(e => console.log("NavigationError:", e));
  }

  async handleSubmit(event?: Event) {
    console.log("In component, handleSubmit()");
    event?.preventDefault();

    if (this.isLoading()) return;
    this.isLoading.set(true);

    try {
      const response = await firstValueFrom(
        this.LoginService.login(this.email, this.password)
      );
      console.log("In component, Login success:", response, "Before router.navigate()");
      const ok = await this.router.navigate(['/main'],);
      console.log("After router.navigate() ok:", ok);

    } catch (err: unknown) {
      if (err instanceof HttpErrorResponse) {

        // If it's an HTTPError, status wills still exist
        if (err.status === 401) {
          this.errorMessage.set("Invalid username or password");
        } else {
          const serverMsg =
            (err.error && (err.error.message || err.error.error || err.error)) || null;
          const err_msg =
            (typeof serverMsg === 'string' && serverMsg.trim()) ||
            'Login failed (HTTP $(e.status)). Please try again later.';
          this.errorMessage.set(err_msg);
        }
      } else {
        // Non HttpErrorResponse, so it's a different error
        this.errorMessage.set("Login failed. Please try again later.");
      }
      console.error("Login failed:", err);
    } finally {
      this.isLoading.set(false);
      console.log('isLoading now:', this.isLoading());
    }
  }
}
