import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DentistClinicsDialogComponent } from './dentist-clinics-dialog-component';

describe('DentistClinicsDialogComponent', () => {
  let component: DentistClinicsDialogComponent;
  let fixture: ComponentFixture<DentistClinicsDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DentistClinicsDialogComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DentistClinicsDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
