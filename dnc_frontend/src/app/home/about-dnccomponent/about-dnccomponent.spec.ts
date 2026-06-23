import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AboutDNCComponent } from './about-dnccomponent';

describe('AboutDNCComponent', () => {
  let component: AboutDNCComponent;
  let fixture: ComponentFixture<AboutDNCComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AboutDNCComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AboutDNCComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
