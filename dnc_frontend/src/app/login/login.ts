import { Component } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import {MatIconModule} from "@angular/material/icon";
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import {LoginService} from "../login.service";
import {firstValueFrom} from 'rxjs';
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
  constructor(private LoginService: LoginService){}

  async handleSubmit(){
    this.isLoading = true;

    try{
      const response = await firstValueFrom(this.LoginService.login(this.email, this.password));
      console.log("In component, Login success:", response);

    } catch(e:any){
      console.log("In component, Login failed:", e.message);
    } finally {
      this.isLoading = false;
    }


  }
}
