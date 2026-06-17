import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DentistClaimsMatrixComponent } from './dentist-claims-matrix-component';

describe('DentistClaimsMatrixComponent', () => {
  let component: DentistClaimsMatrixComponent;
  let fixture: ComponentFixture<DentistClaimsMatrixComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DentistClaimsMatrixComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DentistClaimsMatrixComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
