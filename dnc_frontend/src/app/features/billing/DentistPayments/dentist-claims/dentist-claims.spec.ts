import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DentistClaims } from './dentist-claims';

describe('DentistClaims', () => {
  let component: DentistClaims;
  let fixture: ComponentFixture<DentistClaims>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DentistClaims]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DentistClaims);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
