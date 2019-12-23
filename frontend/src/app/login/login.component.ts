import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup } from '@angular/forms';
import { HttpClient } from "@angular/common/http";
import { AuthenticationService } from '../authentication.service';
import { first } from 'rxjs/operators';
import { ToastService } from '../toast.service';
import { Router } from "@angular/router"

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
    public toastService: ToastService,
    private router: Router
  ) { 
    this.loginForm = this.formBuilder.group({
      email: '',
      password: ''
    });
  }

  ngOnInit() {

  }

  onSubmit(loginData) {
    if (this.loginForm.invalid) {
      return;
    }

    let ts = this.toastService;

    this.authService.login(loginData.email, loginData.password)
      .pipe(first())
      .subscribe(
        data => {
          this.router.navigate(['/']);
        },
        error => {
          ts.show(error.error, { classname: "bg-danger text-light" });
        }
      );
  }

}
