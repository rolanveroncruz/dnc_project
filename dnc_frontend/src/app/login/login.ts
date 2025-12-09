import { Component } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import {LoginService} from "../login.service";
@Component({
  selector: 'app-login',
  standalone: true,
  imports: [
    MatCardModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    FormsModule,
    ReactiveFormsModule,
  ],
  templateUrl: './login.html',
  styleUrl: './login.scss',
})
export class Login {
  email: string = "";
  password: string = "";
  constructor(private LoginService: LoginService){}

  handleSubmit(){
    console.log("submit")
    this.LoginService.login(this.email, this.password).subscribe(res => {
      console.log(res)
    }, err => {
      console.log(err)
    });

  }
}
