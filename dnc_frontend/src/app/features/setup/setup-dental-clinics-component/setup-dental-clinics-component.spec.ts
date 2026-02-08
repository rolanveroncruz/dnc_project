import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetupDentalClinicsComponent } from './setup-dental-clinics-component';

describe('SetupDentalClinicsComponent', () => {
  let component: SetupDentalClinicsComponent;
  let fixture: ComponentFixture<SetupDentalClinicsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SetupDentalClinicsComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SetupDentalClinicsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
