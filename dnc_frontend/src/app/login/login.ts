import { Component } from '@angular/core';
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
  isLoading: boolean = false;
  showPassword: boolean = false;
  constructor(private LoginService: LoginService, private router: Router){
    router.events
      .pipe(filter(e=> e instanceof NavigationError))
      .subscribe(e=> console.log("NavigationError:", e));
  }

  async handleSubmit(){
    this.isLoading = true;

    try {
      const response = await firstValueFrom(this.LoginService.login(this.email, this.password));
      console.log("In component, Login success:", response, "Before router.navigate()");
    }
    catch(e:any){
      console.log("In component, Login failed:", e.message);

    }
    try{
      const ok = await this.router.navigate(['/main'],);
      console.log("After router.navigate() ok:", ok);
    } catch(e:any){
      console.log("router.navigate() failed:", e.message);
    } finally {
      this.isLoading = false;
    }


  }
}
