import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { OfficerBoardComponent } from './officer-board.component';

describe('OfficerBoardComponent', () => {
  let component: OfficerBoardComponent;
  let fixture: ComponentFixture<OfficerBoardComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ OfficerBoardComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(OfficerBoardComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
