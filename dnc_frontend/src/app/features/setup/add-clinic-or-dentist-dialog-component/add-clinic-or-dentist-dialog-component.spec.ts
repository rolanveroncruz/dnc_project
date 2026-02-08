import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AddClinicOrDentistDialogComponent } from './add-clinic-or-dentist-dialog-component';

describe('AddClinicOrDentistDialogComponent', () => {
  let component: AddClinicOrDentistDialogComponent;
  let fixture: ComponentFixture<AddClinicOrDentistDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AddClinicOrDentistDialogComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AddClinicOrDentistDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
