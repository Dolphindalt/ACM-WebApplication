import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup } from '@angular/forms';
import { HttpClient } from "@angular/common/http";
import { ToastService } from '../toast.service';
import { Router } from "@angular/router"

@Component({
  selector: 'app-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.sass']
})
export class RegisterComponent implements OnInit {

  private registerForm: FormGroup;

  constructor(
    private formBuilder: FormBuilder,
    private http: HttpClient,
    public toastService: ToastService,
    private router: Router
  ) {
    this.registerForm = this.formBuilder.group({
      email: '',
      firstName: '',
      lastName: '',
      password: ''
    });
  }

  ngOnInit() {

  }

  onSubmit(registerData) {
    if (this.registerForm.invalid) {
      return;
    }

    let ts = this.toastService;

    this.http.post("auth/register",
      {
        "email": registerData.email,
        "password": registerData.password,
        "first_name": registerData.firstName,
        "last_name": registerData.lastName
      }
    )
    .subscribe(
      (val) => {
        ts.show("Registration successful. Check your email to verify your account.", { classname: "bg-success text-light" });
        this.router.navigate(['/']);
      },
      error => {
        ts.show(error.error, { classname: "bg-danger text-light" });
      }
    );
  }

}
