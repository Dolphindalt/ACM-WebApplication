import { Component, OnInit, ViewChild } from '@angular/core';
import { FormBuilder, FormGroup } from '@angular/forms';
import { HttpClient } from "@angular/common/http";
import { TempAlertComponent } from "../temp-alert/temp-alert.component";
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';

@Component({
  selector: 'app-login-pop-up',
  templateUrl: './login-pop-up.component.html',
  styleUrls: ['./login-pop-up.component.sass'],
})
export class LoginPopUpComponent implements OnInit {

  @ViewChild(TempAlertComponent, {static: false})
  private temp_alert: TempAlertComponent;
  private login_alert_type: string;

  loginForm: FormGroup;
  staticAlertClosed: boolean = true;

  constructor(
    private formBuilder: FormBuilder,
    private http: HttpClient,
    private modalService: NgbModal
    ) {
    this.loginForm = this.formBuilder.group({
      email: '',
      password: ''
    });
  }

  ngOnInit() {
    
  }

  openLogin(content) {
    this.modalService.open(content, {ariaLabelledBy: 'login-modal'});
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
      obj.login_alert_type = "success";
      obj.temp_alert.changeMessage("Login successful.");
      this.modalService.dismissAll();
    },
    response => {
      obj.login_alert_type = "danger";
      obj.temp_alert.changeMessage("Invalid username or password.");
    });
    this.loginForm.reset();
  }

}
