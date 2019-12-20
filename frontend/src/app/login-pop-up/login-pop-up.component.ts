import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup } from '@angular/forms';
import { HttpClient } from "@angular/common/http";

@Component({
  selector: 'app-login-pop-up',
  templateUrl: './login-pop-up.component.html',
  styleUrls: ['./login-pop-up.component.sass']
})
export class LoginPopUpComponent implements OnInit {

  visible: boolean = false;
  loginForm: FormGroup;
  loginMessage: string = "";

  constructor(
    private formBuilder: FormBuilder,
    private http: HttpClient
    ) {
    this.loginForm = this.formBuilder.group({
      email: '',
      password: ''
    });
  }

  ngOnInit() {

  }

  toggleLogin() {
    this.visible = !this.visible;
  }

  onSubmit(loginData) {
    let obj = this;
    this.http.post("http://localhost:8000/auth/login",
    {
      "email": loginData.email,
      "password": loginData.password
    })
    .subscribe((val) => {
      localStorage.setItem("access_token", val["token"]);
      obj.visible = false;
    },
    response => {
      this.loginMessage = "Invalid username or password";
    });
    this.loginForm.reset();
  }

}
