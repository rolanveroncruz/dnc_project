import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DentalClinicComponent } from './dental-clinic-component';

describe('DentalClinicComponent', () => {
  let component: DentalClinicComponent;
  let fixture: ComponentFixture<DentalClinicComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DentalClinicComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DentalClinicComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
