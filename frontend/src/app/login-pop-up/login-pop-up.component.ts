import { Component, OnInit, ViewChild } from '@angular/core';
import { FormBuilder, FormGroup } from '@angular/forms';
import { HttpClient } from "@angular/common/http";
import { TempAlertComponent } from "../temp-alert/temp-alert.component";
 
@Component({
  selector: 'app-login-pop-up',
  templateUrl: './login-pop-up.component.html',
  styleUrls: ['./login-pop-up.component.sass'],
})
export class LoginPopUpComponent implements OnInit {

  @ViewChild(TempAlertComponent, {static: false})
  private temp_alert: TempAlertComponent;
  private login_alert_type: string;

  visible: boolean = false;
  loginForm: FormGroup;
  staticAlertClosed: boolean = true;

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
      obj.login_alert_type = "success";
      obj.temp_alert.changeMessage("Login successful.")
    },
    response => {
      obj.login_alert_type = "danger";
      obj.temp_alert.changeMessage("Invalid username or password.");
    });
    this.loginForm.reset();
  }

}
