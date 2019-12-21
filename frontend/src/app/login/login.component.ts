import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup } from '@angular/forms';
import { HttpClient } from "@angular/common/http";
import { AuthenticationService } from '../authentication.service';
import { first } from 'rxjs/operators';
import { ToastService } from '../toast.service';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.sass']
})
export class LoginComponent implements OnInit {

  private loginForm: FormGroup;

  constructor(
    private formBuilder: FormBuilder,
    private http: HttpClient,
    private authService: AuthenticationService,
    public toastService: ToastService
  ) { 
    this.loginForm = this.formBuilder.group({
      email: '',
      password: ''
    });
  }

  ngOnInit() {

  }

  onSubmit(loginData) {
    
    if(this.loginForm.invalid) {
      return;
    }

    let ts = this.toastService;

    this.authService.login(loginData.email, loginData.password)
      .pipe(first())
      .subscribe(
        data => {
          
        },
        error => {
          ts.show("Invalid username or password.", { classname: "bg-danger text-light" });
        }
      );
    this.loginForm.reset();
  }

}
