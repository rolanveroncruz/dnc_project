import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ClinicCapabilities } from './clinic-capabilities';

describe('SetupClinicCapabilities', () => {
  let component: ClinicCapabilities;
  let fixture: ComponentFixture<ClinicCapabilities>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ClinicCapabilities]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ClinicCapabilities);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
